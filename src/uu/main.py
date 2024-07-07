strd = """
// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.


pub mod {folder};
pub mod uu_args;
pub use uu_args::uu_app;

pub use {folder}::uumain;

"""


def job():
    # list folders in the current directory
    import os

    print(os.listdir())
    # filter out folders
    folders = [f for f in os.listdir() if os.path.isdir(f)]
    print(folders)
    for folder in folders:
        # check if src/lib.rs exists

        if os.path.exists(f"{folder}/src/lib.rs"):
            print(f"{folder} has src/lib.rs")
        else:
            # remove [lib] part from Cargo.toml
            with open(f"{folder}/Cargo.toml", "r") as f:
                data = f.read()
                # find the [lib] part
                start = data.find("[lib]")
                end = data.find('"\n\n', start)
                # remove the [lib] part
            data = data[:start] + data[end + 2 :]
            with open(f"{folder}/Cargo.toml", "w") as f:
                f.write(data)
            # create src/lib.rs
            with open(f"{folder}/src/lib.rs", "w") as f:
                f.write(strd.format(folder=folder))
            # grep the uu_app function from src/{folder}.rs
            with open(f"{folder}/src/{folder}.rs", "r") as f:
                data = f.read()
                start = data.find("pub fn uu_app(")
                end = data.find("}", start)
                with open(f"{folder}/src/uu_args.rs", "w") as f:
                    f.write(data[start : end + 1])
            # remove the uu_app function from src/{folder}.rs
            data = data[:start] + data[end + 1 :]
            with open(f"{folder}/src/{folder}.rs", "w") as f:
                f.write(data)


job()
