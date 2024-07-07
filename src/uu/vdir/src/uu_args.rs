// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use uu_ls;

// To avoid code duplication, we reuse ls uu_app function which has the same
// arguments. However, coreutils won't compile if one of the utils is missing
// an uu_app function, so we need this dummy one.
use clap::Command;
pub fn uu_app() -> Command {
    uu_ls::uu_app()
}
