mod io;
mod log;
mod parser;
mod rnodes;
mod utils;
mod visitor;

use log::logger::ILogger;
use utils::cli_args::CLIArgs;

use crate::io::reader::RsonReader;

#[allow(unused_imports)]
use crate::io::writer::RsonWriter;

extern crate chrono;

#[macro_use]
extern crate downcast_rs;

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    let mut cli_args = CLIArgs::new();
    let opt_cli_result: Option<(i32, String)> = cli_args.parse(&args);

    if opt_cli_result.is_some()
    {
        let (err_code, msg) = opt_cli_result.unwrap();

        if err_code == 0
        {
            println!("{}", msg);
            std::process::exit(err_code);
        }

        else
        {
            let logger_cell = log::logger::get_std_logger().lock().unwrap();
            let mut logger = logger_cell.borrow_mut();
            logger.fatal(msg, Some(err_code));
        }
    }

    let rson_reader: RsonReader = match cli_args.input_file {
        Some(input_file) => { RsonReader::from_file(&input_file, cli_args.stringify) },
        None => { RsonReader::from_stdin(cli_args.stringify) }
    };

    let root_node_result = rson_reader.parse();

    match root_node_result
    {
        Ok(root_node) =>
        {
            let enum_type = root_node.get_node_type();
            let logger_cell = log::logger::get_std_logger().lock().unwrap();
            let mut logger = logger_cell.borrow_mut();
            logger.info(enum_type.to_string());
        },
        Err(msg) =>
        {
            let logger_cell = log::logger::get_std_logger().lock().unwrap();
            let mut logger = logger_cell.borrow_mut();
            logger.error(msg);

            // "Normal" error in parsing will be >0
            std::process::exit(1);
        }
    }
}

