// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use log::{error, warn, info, debug, trace};

mod elephant_logger;
mod zfs;

use zfs::filesystem::Filesystem;

fn main() {

    elephant_logger::ElephantLogger::init().unwrap();

    let number = 9;

    println!("Hello, world!{}", number);

    error!("log error message");
    warn!("log warn message");
    info!("log info message");
    debug!("log debug message");
    trace!("log trace message");

    archive();

    info!("Finished Elephant Archive.");
}

fn archive() {
    let filesystem = Filesystem::init();
    let snapshots = filesystem.get_snapshots();

    info!("Snapshots: {}", snapshots[0]);
}
