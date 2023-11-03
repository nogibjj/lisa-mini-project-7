//use std::env;

mod extract;
mod transform_load;
mod query;

use std::time::{Instant, Duration};
//use std::process::Command;
//use std::io::Write;
//use std::fs::File;
use procfs::process::Process;


use sysinfo::{ProcessExt, System, SystemExt, PidExt, Pid};



fn run_all_metrics() -> Result<(), std::io::Error> {
    let mut system = sysinfo::System::new_all();

    system.refresh_all();

    let start = Instant::now();

    extract::run().expect("Failed to run extract");
    transform_load::main().expect("Failed to run transform_load");
    query::query().expect("Failed to run query");

    let duration = start.elapsed();

    let me = Process::myself().unwrap();
    let me_stat = me.stat().unwrap();
    let page_size = procfs::page_size();

    println!("== Data from /proc/self/stat:");
    println!("Total virtual memory used: {} MB", me_stat.vsize /1000000);
    println!("Time elapsed in extract, transform and query is: {:?}", duration);

    

    //println!("Total resident set size: {} MB", me_stat.rss * page_size / 1024);

    
    Ok(())
    // After extract::run()
    // let duration_extract = start_extract.elapsed();
    // system.refresh_all();
    // let after_extract = system.process(libc::getpid() as sysinfo::Pid);
    
    // if let (Some(before), Some(after)) = (before_extract, after_extract) {
    //     println!("Time elapsed in extract::run() is: {:?}", duration_extract);
    //     println!("CPU usage in extract::run() is: {}", after.cpu_usage() - before.cpu_usage());
    //     println!("Memory usage in extract::run() is: {}", after.memory() - before.memory());
    // } else {
    //     println!("Failed to get process info");
    // }
}



pub fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("extract") => { extract::run().expect("Failed to run extract"); },
        Some("transform_load") => { transform_load::main().expect("Failed to run transform_load"); },
        Some("query") => { query::query().expect("Failed to run query"); },
        Some("all") => {
            extract::run().expect("Failed to run extract");
            transform_load::main().expect("Failed to run transform_load");
            query::query().expect("Failed to run query");
        },
        Some("all_metrics") => {
            run_all_metrics().expect("Failed to run all_20");
        },
        _ => println!("Invalid command provided"),
    };
}