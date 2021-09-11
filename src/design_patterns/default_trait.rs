use std::{path::PathBuf, time::Duration};

// Many type of Rust has constructor.
// But this property depends on each implementation and
// can't abstract it like 'all type has new() method'
//
// So, 'Default' trait is developed
// You can automatically derive in container type Cow/Box/Arc
// when it's elememnt is satisfies 'Default' trait
//
// warn: Default trait's default() methods has no parameters

#[derive(Default, Debug)]
struct MyConfiguration {
    // Default of Option type is None
    output: Option<RawBuf>,
    // Default of Vec type is empty vector
    search_path: Vec<PathBuf>,
    // Default of Duration is zero time
    timeout: Duration,
    // Default of bool is false,
    check: bool,
}

impl MyConfiguration {
    // add setter from this.
}

pub fn setup_configuration() {
    // create new instance of MyConfiguration that has default value
    let mut conf = MyConfiguration::default();
    // anything to do using conf
    conf.check = true;
    println!("conf = {:#?}", conf);
}
