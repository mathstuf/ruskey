extern crate wm_daemons;
use wm_daemons::config::{load_config, load_config_path};
use wm_daemons::dbus_listen::{CallbackMap, match_method};

#[macro_use]
extern crate clap;
use clap::{Arg, App};

extern crate config;
use self::config::types::Config;

extern crate dbus;
use self::dbus::{Connection, BusType};

use std::error::Error;
use std::path::Path;

fn try_main() -> Result<(), Box<Error>> {
    let matches = App::new("ruskey")
        .version(&crate_version!()[..])
        .author("Ben Boeckel <mathstuf@gmail.com>")
        .about("Implement the SecretService API for keepass databases")
        .arg(Arg::with_name("CONFIG")
                .short("c")
                .long("config")
                .help("Path to the configuration file")
                .takes_value(true))
        .arg(Arg::with_name("DATABASE_NAME")
                .short("n")
                .long("name")
                .help("Name of the database")
                .takes_value(true))
        .arg(Arg::with_name("DATABASE")
                .short("d")
                .long("database")
                .help("Path to the database")
                .takes_value(true))
        .get_matches();

    let conf = try!(if matches.is_present("CONFIG") {
            load_config_path(Path::new(matches.value_of("CONFIG").unwrap()))
        } else {
            load_config("ruskey", "config")
        });

    let conn = try!(Connection::get_private(BusType::Session));

    let cbs: CallbackMap<Config> = vec![
        // TODO
    ];

    let match_str = "";
    try!(conn.add_match(match_str));

    for items in conn.iter(100) {
        match_method(items, &cbs, &conf);
    }

    Ok(())
}

fn main() {
    if let Err(err) = try_main() {
        panic!("{}", err.description());
    }
}
