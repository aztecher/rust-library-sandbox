// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's "builder pattern" method of creating arguments
// which the most flexible, but also most verbose.
use clap::{Arg, App};
// use self::my_serde::sample;
use rust_library_sandbox::my_serde::sample as serde_sample;
use rust_library_sandbox::slack_api::client as slack_client;
use rust_library_sandbox::design_patterns::ideom;

fn main() {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("aztecher <mikiyaf.business@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .about("Sets a custom config file")
            .takes_value(true))
        // .arg(Arg::new("INPUT")
        //     .about("Sets the input file to use")
        //     .required(true)
        //     .index(1))
        .arg(Arg::new("v")
            .short('v')
            .multiple_occurrences(true)
            .takes_value(true)
            .about("Sets the level of verbosity"))
        .subcommand(App::new("test")
            .about("controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg(Arg::new("debug")
                .short('d')
                .about("print debug information verbosely")))
        .subcommand(App::new("serde")
            .about("Example Serde package")
            .arg(Arg::new("example")
                .takes_value(true)
                .about("Sample Serde serialize/deserialize")))
        .subcommand(App::new("slack")
            .about("Slack example")
            .subcommand(App::new("post")
                .about("Example of posting slack message")
                .arg(Arg::new("config")
                     .short('c')
                     .long("config")
                     .value_name("FILE")
                     .about("Slack config file")
                     .required(true))))
        .subcommand(App::new("design-patterns")
            .about("Example design pattenr")
            .arg(Arg::new("type-coercions")
                .about("Example of design patterns of type coercions")
                .takes_value(true)))
        .get_matches();

    // // You can check the value provided by positional arguments, or option arguments
    // if let Some(i) = matches.value_of("INPUT") {
    //     println!("Value for input: {}", i);
    // }

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("v") {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(ref matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.is_present("debug") {
            // "$ myapp test -d" was run
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    } else if let Some(ref matches) = matches.subcommand_matches("serde") {
        if matches.is_present("example") {
            serde_sample::serialize_deserialize_example();
        } else {
            println!("Serde not work because of unexpected command parameters ...")
        }
    } else if let Some(ref matches) = matches.subcommand_matches("slack") {
        if let Some(ref matches) = matches.subcommand_matches("post") {
            if let Some(config) = matches.value_of("config") {
                println!("Read config file : {config}", config=config);
                match slack_client::post_from_file(config) {
                    Ok(_) => return,
                    Err(e) => println!("error while posting to slack api: {error}", error=e),
                }
            }
        }
    } else if let Some(ref matches) = matches.subcommand_matches("design-patterns") {
        if matches.is_present("type-coercions") {
            let ferris = "Ferris".to_string();
            let curious = "Curious".to_string();
            println!("{}: {}", ferris, ideom::three_vowels(&ferris));
            println!("{}: {}", curious, ideom::three_vowels(&curious));
        }
    }

    // Continued program logic goes here...
}

