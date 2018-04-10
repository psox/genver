#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("version")
                .about("modify the version part of the string")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("inc")
                        .about("roll the version of a component")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .arg(
                            Arg::with_name("part")
                                .help("which component should be incremented")
                                .index(1)
                                .required(true),
                        ),
                ),
        )
        .get_matches();

    println!("{:#?}", matches);
    // Continued program logic goes here...
}
