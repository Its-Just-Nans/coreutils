// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

// spell-checker:ignore RUNLEVEL runlevel MESG mesg

pub mod uu_args;
pub mod who;
pub use uu_args::uu_app;

pub use who::uumain;

mod platform;

mod options {
    pub const ALL: &str = "all";
    pub const BOOT: &str = "boot";
    pub const DEAD: &str = "dead";
    pub const HEADING: &str = "heading";
    pub const LOGIN: &str = "login";
    pub const LOOKUP: &str = "lookup";
    pub const ONLY_HOSTNAME_USER: &str = "only_hostname_user";
    pub const PROCESS: &str = "process";
    pub const COUNT: &str = "count";
    pub const RUNLEVEL: &str = "runlevel";
    pub const SHORT: &str = "short";
    pub const TIME: &str = "time";
    pub const USERS: &str = "users";
    pub const MESG: &str = "mesg"; // aliases: --message, --writable
    pub const FILE: &str = "FILE"; // if length=1: FILE, if length=2: ARG1 ARG2
}
