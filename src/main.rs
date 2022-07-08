mod cli;
mod ctr_logging;

use clap::Parser;
// use log::*;
use anyhow::Result;

fn app() -> Result<()>{
    // initialize cli
    let mut args = cli::Cli::parse();
    args.process_cli()?;

    // initialize logging
    stderrlog::new()
        .module(module_path!())
        .quiet(args.quiet)
        .verbosity(args.verbosity)
        .init()
        .unwrap();


    // println!("{:?}", args.runtime_arg);
    // println!("{}", args.container_pidfile);

    // trace!("trace message");
    // debug!("debug message");
    // info!("info message");
    // warn!("warn message");
    // error!("error message");

    ctr_logging::configure_log_drivers(args.log_path, args.log_size_max, args.cid, args.name, args.log_tag);
    

    Ok(())
}

fn main() {
    if let Err(e) = app() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
