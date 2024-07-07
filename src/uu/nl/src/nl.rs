// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use clap::{crate_version, Arg, ArgAction, Command};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read};
use std::path::Path;
use uucore::error::{set_exit_code, FromIo, UResult, USimpleError};
use uucore::{format_usage, help_about, help_section, help_usage, show_error};

// Settings store options used by nl to produce its output.
pub struct Settings {
    // The variables corresponding to the options -h, -b, and -f.
    pub(crate) header_numbering: NumberingStyle,
    pub(crate) body_numbering: NumberingStyle,
    pub(crate) footer_numbering: NumberingStyle,
    // The variable corresponding to -d
    pub(crate) section_delimiter: String,
    // The variables corresponding to the options -v, -i, -l, -w.
    pub(crate) starting_line_number: i64,
    pub(crate) line_increment: i64,
    pub(crate) join_blank_lines: u64,
    pub(crate) number_width: usize, // Used with String::from_char, hence usize.
    // The format of the number and the (default value for)
    // renumbering each page.
    pub(crate) number_format: NumberFormat,
    pub(crate) renumber: bool,
    // The string appended to each line number output.
    pub(crate) number_separator: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            header_numbering: NumberingStyle::None,
            body_numbering: NumberingStyle::NonEmpty,
            footer_numbering: NumberingStyle::None,
            section_delimiter: String::from("\\:"),
            starting_line_number: 1,
            line_increment: 1,
            join_blank_lines: 1,
            number_width: 6,
            number_format: NumberFormat::Right,
            renumber: true,
            number_separator: String::from("\t"),
        }
    }
}

struct Stats {
    line_number: Option<i64>,
    consecutive_empty_lines: u64,
}

impl Stats {
    fn new(starting_line_number: i64) -> Self {
        Self {
            line_number: Some(starting_line_number),
            consecutive_empty_lines: 0,
        }
    }
}

// NumberingStyle stores which lines are to be numbered.
// The possible options are:
// 1. Number all lines
// 2. Number only nonempty lines
// 3. Don't number any lines at all
// 4. Number all lines that match a basic regular expression.
pub(crate) enum NumberingStyle {
    All,
    NonEmpty,
    None,
    Regex(Box<regex::Regex>),
}

impl TryFrom<&str> for NumberingStyle {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "a" => Ok(Self::All),
            "t" => Ok(Self::NonEmpty),
            "n" => Ok(Self::None),
            _ if s.starts_with('p') => match regex::Regex::new(&s[1..]) {
                Ok(re) => Ok(Self::Regex(Box::new(re))),
                Err(_) => Err(String::from("invalid regular expression")),
            },
            _ => Err(format!("invalid numbering style: '{s}'")),
        }
    }
}

// NumberFormat specifies how line numbers are output within their allocated
// space. They are justified to the left or right, in the latter case with
// the option of having all unused space to its left turned into leading zeroes.
#[derive(Default)]
pub(crate) enum NumberFormat {
    Left,
    #[default]
    Right,
    RightZero,
}

impl<T: AsRef<str>> From<T> for NumberFormat {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "ln" => Self::Left,
            "rn" => Self::Right,
            "rz" => Self::RightZero,
            _ => unreachable!("Should have been caught by clap"),
        }
    }
}

impl NumberFormat {
    // Turns a line number into a `String` with at least `min_width` chars,
    // formatted according to the `NumberFormat`s variant.
    fn format(&self, number: i64, min_width: usize) -> String {
        match self {
            Self::Left => format!("{number:<min_width$}"),
            Self::Right => format!("{number:>min_width$}"),
            Self::RightZero if number < 0 => format!("-{0:0>1$}", number.abs(), min_width - 1),
            Self::RightZero => format!("{number:0>min_width$}"),
        }
    }
}

enum SectionDelimiter {
    Header,
    Body,
    Footer,
}

impl SectionDelimiter {
    // A valid section delimiter contains the pattern one to three times,
    // and nothing else.
    fn parse(s: &str, pattern: &str) -> Option<Self> {
        if s.is_empty() || pattern.is_empty() {
            return None;
        }

        let pattern_count = s.matches(pattern).count();
        let is_length_ok = pattern_count * pattern.len() == s.len();

        match (pattern_count, is_length_ok) {
            (3, true) => Some(Self::Header),
            (2, true) => Some(Self::Body),
            (1, true) => Some(Self::Footer),
            _ => None,
        }
    }
}

#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let matches = crate::uu_app().try_get_matches_from(args)?;

    let mut settings = Settings::default();

    // Update the settings from the command line options, and terminate the
    // program if some options could not successfully be parsed.
    let parse_errors = crate::helper::parse_options(&mut settings, &matches);
    if !parse_errors.is_empty() {
        return Err(USimpleError::new(
            1,
            format!("Invalid arguments supplied.\n{}", parse_errors.join("\n")),
        ));
    }

    let files: Vec<String> = match matches.get_many::<String>(crate::options::FILE) {
        Some(v) => v.cloned().collect(),
        None => vec!["-".to_owned()],
    };

    let mut stats = Stats::new(settings.starting_line_number);

    for file in &files {
        if file == "-" {
            let mut buffer = BufReader::new(stdin());
            nl(&mut buffer, &mut stats, &settings)?;
        } else {
            let path = Path::new(file);

            if path.is_dir() {
                show_error!("{}: Is a directory", path.display());
                set_exit_code(1);
            } else {
                let reader = File::open(path).map_err_context(|| file.to_string())?;
                let mut buffer = BufReader::new(reader);
                nl(&mut buffer, &mut stats, &settings)?;
            }
        }
    }

    Ok(())
}

// nl implements the main functionality for an individual buffer.
fn nl<T: Read>(reader: &mut BufReader<T>, stats: &mut Stats, settings: &Settings) -> UResult<()> {
    let mut current_numbering_style = &settings.body_numbering;

    for line in reader.lines() {
        let line = line.map_err_context(|| "could not read line".to_string())?;

        if line.is_empty() {
            stats.consecutive_empty_lines += 1;
        } else {
            stats.consecutive_empty_lines = 0;
        };

        let new_numbering_style = match SectionDelimiter::parse(&line, &settings.section_delimiter)
        {
            Some(SectionDelimiter::Header) => Some(&settings.header_numbering),
            Some(SectionDelimiter::Body) => Some(&settings.body_numbering),
            Some(SectionDelimiter::Footer) => Some(&settings.footer_numbering),
            None => None,
        };

        if let Some(new_style) = new_numbering_style {
            current_numbering_style = new_style;
            if settings.renumber {
                stats.line_number = Some(settings.starting_line_number);
            }
            println!();
        } else {
            let is_line_numbered = match current_numbering_style {
                // consider $join_blank_lines consecutive empty lines to be one logical line
                // for numbering, and only number the last one
                NumberingStyle::All
                    if line.is_empty()
                        && stats.consecutive_empty_lines % settings.join_blank_lines != 0 =>
                {
                    false
                }
                NumberingStyle::All => true,
                NumberingStyle::NonEmpty => !line.is_empty(),
                NumberingStyle::None => false,
                NumberingStyle::Regex(re) => re.is_match(&line),
            };

            if is_line_numbered {
                let Some(line_number) = stats.line_number else {
                    return Err(USimpleError::new(1, "line number overflow"));
                };
                println!(
                    "{}{}{}",
                    settings
                        .number_format
                        .format(line_number, settings.number_width),
                    settings.number_separator,
                    line
                );
                // update line number for the potential next line
                match line_number.checked_add(settings.line_increment) {
                    Some(new_line_number) => stats.line_number = Some(new_line_number),
                    None => stats.line_number = None, // overflow
                }
            } else {
                let spaces = " ".repeat(settings.number_width + 1);
                println!("{spaces}{line}");
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_format() {
        assert_eq!(NumberFormat::Left.format(12, 1), "12");
        assert_eq!(NumberFormat::Left.format(-12, 1), "-12");
        assert_eq!(NumberFormat::Left.format(12, 4), "12  ");
        assert_eq!(NumberFormat::Left.format(-12, 4), "-12 ");

        assert_eq!(NumberFormat::Right.format(12, 1), "12");
        assert_eq!(NumberFormat::Right.format(-12, 1), "-12");
        assert_eq!(NumberFormat::Right.format(12, 4), "  12");
        assert_eq!(NumberFormat::Right.format(-12, 4), " -12");

        assert_eq!(NumberFormat::RightZero.format(12, 1), "12");
        assert_eq!(NumberFormat::RightZero.format(-12, 1), "-12");
        assert_eq!(NumberFormat::RightZero.format(12, 4), "0012");
        assert_eq!(NumberFormat::RightZero.format(-12, 4), "-012");
    }
}
