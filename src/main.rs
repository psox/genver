#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new("git")
        .about("A fictional versioning CLI")
        .version("1.0")
        .author("Me")
        .subcommand(
            SubCommand::with_name("clone").about("clones repos").arg(
                Arg::with_name("repo")
                    .help("The repo to clone")
                    .required(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("push")
                .about("pushes things")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("remote")  // Subcommands can have their own subcommands,
                                                     // which in turn have their own subcommands
                .about("pushes remote things")
                .arg(Arg::with_name("repo")
                    .required(true)
                    .help("The remote repo to push things to")),
                )
                .subcommand(SubCommand::with_name("local").about("pushes local things")),
        )
        .subcommand(
            SubCommand::with_name("add")
            .about("adds things")
            .author("Someone Else")                     // Subcommands can list different authors
            .version("v2.0 (I'm versioned differently") // or different version from their parents
            .setting(AppSettings::ArgRequiredElseHelp)  // They can even have different settings
            .arg(Arg::with_name("stuff")
                .long("stuff")
                .help("Stuff to add")
                .takes_value(true)
                .multiple(true)),
        )
        .get_matches();

    // At this point, the matches we have point to git. Keep this in mind...

    // You can check if one of git's subcommands was used
    if matches.is_present("clone") {
        println!("'git clone' was run.");
    }

    // You can see which subcommand was used
    if let Some(subcommand) = matches.subcommand_name() {
        println!("'git {}' was used", subcommand);

        // It's important to note, this *only* check's git's DIRECT children, **NOT** it's
        // grandchildren, great grandchildren, etc.
        //
        // i.e. if the command `git push remove --stuff foo` was run, the above will only print out,
        // `git push` was used. We'd need to get push's matches to see further into the tree
    }

    // An alternative to checking the name is matching on known names. Again notice that only the
    // direct children are matched here.
    match matches.subcommand_name() {
        Some("clone") => println!("'git clone' was used"),
        Some("push") => println!("'git push' was used"),
        Some("add") => println!("'git add' was used"),
        None => println!("No subcommand was used"),
        _ => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
    }

    // You could get the independent subcommand matches, although this is less common
    if let Some(clone_matches) = matches.subcommand_matches("clone") {
        // Now we have a reference to clone's matches
        println!("Cloning repo: {}", clone_matches.value_of("repo").unwrap());
    }

    // The most common way to handle subcommands is via a combined approach using
    // `ArgMatches::subcommand` which returns a tuple of both the name and matches
    match matches.subcommand() {
        ("clone", Some(clone_matches)) => {
            // Now we have a reference to clone's matches
            println!("Cloning {}", clone_matches.value_of("repo").unwrap());
        }
        ("push", Some(push_matches)) => {
            // Now we have a reference to push's matches
            match push_matches.subcommand() {
                ("remote", Some(remote_matches)) => {
                    // Now we have a reference to remote's matches
                    println!("Pushing to {}", remote_matches.value_of("repo").unwrap());
                }
                ("local", Some(_)) => {
                    println!("'git push local' was used");
                }
                _ => unreachable!(),
            }
        }
        ("add", Some(add_matches)) => {
            // Now we have a reference to add's matches
            println!(
                "Adding {}",
                add_matches
                    .values_of("stuff")
                    .unwrap()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        ("", None) => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }

    // Continued program logic goes here...
}
