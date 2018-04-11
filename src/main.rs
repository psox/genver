#[macro_use]
extern crate clap;
extern crate config;

use clap::{App, AppSettings, Arg, SubCommand};
use config::{Config, Value};

fn main() {
    let mut settings = Config::new();
    settings
        .set_default("k1", Value::from(vec![1, 3, 4]))
        .unwrap();

    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("inc")
                .about("increment the ...")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("version")
                        .about("version part")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .arg(
                            Arg::with_name("part")
                                .help("which component should be incremented")
                                .index(1)
                                .required(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("prefix")
                        .about("prefix part")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .arg(
                            Arg::with_name("part")
                                .help("which component should be incremented")
                                .index(1)
                                .required(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("build")
                        .about("build part")
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
    println!("{:#?}", settings);
    // Continued program logic goes here...
}
