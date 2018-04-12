#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use clap::{App, AppSettings, Arg, SubCommand};

#[derive(Serialize, Deserialize, Debug)]
struct MinMax<T> {
    min: T, // minimum number of parts
    max: T, // maximum number of parts
}

#[derive(Serialize, Deserialize, Debug)]
enum EntryType {
    None,                         // nothing
    RegEx(String),                // regular expresion
    Exact(String),                // exact match
    Front(String),                // front part of item
    Back(String),                 // end part of item
    PartType(u8, Box<EntryType>), // specific for this part
    PartName(u8, String),         // the name of this part
}

#[derive(Serialize, Deserialize, Debug)]
struct Section {
    name: String,           // name of the section
    parts: MinMax<u8>,      // minimum and maximum
    flag: Option<char>,     // what signifies the start of this section
    custom: Vec<EntryType>, // customize this section
}

#[derive(Serialize, Deserialize, Debug)]
struct GenVer {
    name: String,           // Name of this configuration
    separator: char,        // this separates parts in all sections
    sections: Vec<Section>, // Describe each section
}

fn main() {
    let defaults = GenVer {
        name: String::from("Semantic Versioning"),
        separator: '.',
        sections: vec![
            Section {
                name: String::from("version"),
                parts: MinMax { min: 3, max: 3 },
                flag: None,
                custom: vec![
                    EntryType::PartType(1, Box::new(EntryType::RegEx(String::from("v?\\d+")))),
                    EntryType::PartName(1, String::from("major")),
                    EntryType::PartType(2, Box::new(EntryType::RegEx(String::from("\\d+")))),
                    EntryType::PartName(2, String::from("minor")),
                    EntryType::PartType(3, Box::new(EntryType::RegEx(String::from("\\d+")))),
                    EntryType::PartName(3, String::from("patch")),
                ],
            },
            Section {
                name: String::from("prefix"),
                parts: MinMax { min: 1, max: 1 },
                flag: Some('-'),
                custom: vec![
                    EntryType::PartType(
                        1,
                        Box::new(EntryType::RegEx(String::from("(alpha|beta|rc)?\\d+"))),
                    ),
                ],
            },
            Section {
                name: String::from("build"),
                parts: MinMax { min: 1, max: 1 },
                flag: Some('+'),
                custom: vec![EntryType::Front(String::from("v"))],
            },
        ],
    };
    let defaults_yaml = serde_yaml::to_string(&defaults).unwrap();
    //let mut settings = Config::new();

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
    println!("{:#?}", defaults);
    println!("{}", defaults_yaml);
    // Continued program logic goes here...
}
