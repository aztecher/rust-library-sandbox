// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's "builder pattern" method of creating arguments
// which the most flexible, but also most verbose.
use clap::{Arg, App};
// use self::my_serde::sample;
use rust_library_sandbox::my_serde::sample as serde_sample;
use rust_library_sandbox::slack_api::client as slack_client;
use rust_library_sandbox::design_patterns::ideom;
use rust_library_sandbox::my_sandbox::sandbox;
use rust_library_sandbox::my_sandbox::smartpointers;
use rust_library_sandbox::my_sandbox::mybox;
use rust_library_sandbox::concurrency;

// load for test
use rust_library_sandbox::design_patterns::format_string;

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
        .subcommand(App::new("sandbox")
            .about("Sandbox")
            .subcommand(App::new("ownership")
                .about("Example of sandbox of ownership"))
            .subcommand(App::new("slice")
                .about("Example of sandbox of slice"))
            .subcommand(App::new("struct_reference")
                .about("Example of reference of struct"))
            .subcommand(App::new("myalonetuple_reference")
                .about("Example of MyAloneTuple"))
            .subcommand(App::new("box_example")
                .about("Example of box"))
            .subcommand(App::new("mybox_example")
                .about("Example of MyBox"))
            .subcommand(App::new("rc_example")
                .about("Example of rc_example"))
            .subcommand(App::new("refcall_example1")
                .about("Example of refcall_example1"))
            .subcommand(App::new("concurrency")
                .about("Example of conccurency codes")
                .subcommand(App::new("mutex")
                    .about("Basic example of Mutex"))
                .subcommand(App::new("condvar")
                    .about("Basic example of Condvar"))
                .subcommand(App::new("rwlock")
                    .about("Basic example of RwLock"))
                .subcommand(App::new("barrier")
                    .about("Basic example of barrier sync"))
                .subcommand(App::new("semaphore")
                    .about("Basic example of semaphore"))
                .subcommand(App::new("channel")
                    .about("Basic example of channel"))
                .subcommand(App::new("bakery-algorithm")
                    .about("Bakery algorithm in Rust"))
                )
            )
            // .arg(Arg::new("ownership")
            //     .about("Example of sandbox of ownership")
            //     .takes_value(true))
            // .arg(Arg::new("slice")
            //     .about("Example of sandbox of slice")
            //     .takes_value(true)))
        // .subcommand(App::new("tokio-sandbox")
        //     .about("Tokio Sandbox")
        //     .subcommand(App::new("simple")
        //         .about("Simple Example of Tokio"))
        //     )
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
    } else if let Some(ref matches) = matches.subcommand_matches("sandbox") {
        if let Some(ref _matches) = matches.subcommand_matches("ownership") {
            sandbox::ownership();
        } else if let Some(ref _matches) = matches.subcommand_matches("slice") {
            sandbox::slice();
        } else if let Some(ref _matches) = matches.subcommand_matches("struct_reference") {
            sandbox::test_mystruct_reference();
        } else if let Some(ref _matches) = matches.subcommand_matches("myalonetuple_reference") {
            sandbox::test_myalonetuple_reference();
        } else if let Some(ref _matches) = matches.subcommand_matches("box_example") {
            smartpointers::box_example();
        } else if let Some(ref _matches) = matches.subcommand_matches("mybox_example") {
            mybox::mybox_example();
        } else if let Some(ref _matches) = matches.subcommand_matches("rc_example") {
            smartpointers::rc_example();
        } else if let Some(ref _matches) = matches.subcommand_matches("refcall_example1") {
            smartpointers::refcall_example1();
        } else if let Some(ref matches) = matches.subcommand_matches("concurrency") {
            if let Some(ref _matches) = matches.subcommand_matches("mutex") {
                concurrency::basic::mutex::mutex_sample();
            }
            else if let Some(ref _matches) = matches.subcommand_matches("condvar") {
                concurrency::basic::condvar::condvar_sample();
            }
            else if let Some(ref _matches) = matches.subcommand_matches("rwlock") {
                concurrency::basic::rwlock::rwlock_sample();
            }
            else if let Some(ref _matches) = matches.subcommand_matches("barrier") {
                concurrency::basic::barrier::barrier_sample();
            }
            else if let Some(ref _matches) = matches.subcommand_matches("semaphore") {
                concurrency::basic::semaphore::semaphore_sample();
            }
            else if let Some(ref _matches) = matches.subcommand_matches("channel") {
                concurrency::channel::channel::channel_sample();
            }
            else if let Some(ref _matches) = matches.subcommand_matches("bakery-algorithm") {
                concurrency::algorithm::bakery::bakery_algorithm_impl();
            }
        }
        // if matches.is_present("ownership") {
        //     sandbox::ownership();
        // } else if matches.is_present("slice") {
        //     sandbox::slice();
        // }
        //
    // } else if let Some(ref matches) = matches.subcommand_matches("tokio-sandbox") {
    //     if let Some(ref _matches) = matches.subcommand_matches("simple") {
    //         tokio_simple::simple()
        // }
    }

    // Continued program logic goes here...
}

