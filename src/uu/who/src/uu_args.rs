// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use clap::{crate_version, Arg, ArgAction, Command};
use uucore::{format_usage, help_about, help_usage};

const ABOUT: &str = help_about!("who.md");
const USAGE: &str = help_usage!("who.md");

#[cfg(target_os = "linux")]
static RUNLEVEL_HELP: &str = "print current runlevel";
#[cfg(not(target_os = "linux"))]
static RUNLEVEL_HELP: &str = "print current runlevel (This is meaningless on non Linux)";

pub fn uu_app() -> Command {
    Command::new(uucore::util_name())
        .version(crate_version!())
        .about(ABOUT)
        .override_usage(format_usage(USAGE))
        .infer_long_args(true)
        .arg(
            Arg::new(crate::options::ALL)
                .long(crate::options::ALL)
                .short('a')
                .help("same as -b -d --login -p -r -t -T -u")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::BOOT)
                .long(crate::options::BOOT)
                .short('b')
                .help("time of last system boot")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::DEAD)
                .long(crate::options::DEAD)
                .short('d')
                .help("print dead processes")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::HEADING)
                .long(crate::options::HEADING)
                .short('H')
                .help("print line of column headings")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::LOGIN)
                .long(crate::options::LOGIN)
                .short('l')
                .help("print system login processes")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::LOOKUP)
                .long(crate::options::LOOKUP)
                .help("attempt to canonicalize hostnames via DNS")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::ONLY_HOSTNAME_USER)
                .short('m')
                .help("only hostname and user associated with stdin")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::PROCESS)
                .long(crate::options::PROCESS)
                .short('p')
                .help("print active processes spawned by init")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::COUNT)
                .long(crate::options::COUNT)
                .short('q')
                .help("all login names and number of users logged on")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::RUNLEVEL)
                .long(crate::options::RUNLEVEL)
                .short('r')
                .help(RUNLEVEL_HELP)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::SHORT)
                .long(crate::options::SHORT)
                .short('s')
                .help("print only name, line, and time (default)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::TIME)
                .long(crate::options::TIME)
                .short('t')
                .help("print last system clock change")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::USERS)
                .long(crate::options::USERS)
                .short('u')
                .help("list users logged in")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::MESG)
                .long(crate::options::MESG)
                .short('T')
                .visible_short_alias('w')
                .visible_aliases(["message", "writable"])
                .help("add user's message status as +, - or ?")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(crate::options::FILE)
                .num_args(1..=2)
                .value_hint(clap::ValueHint::FilePath),
        )
}
