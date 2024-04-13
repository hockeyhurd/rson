mod log;
mod parser;
mod rnodes;
mod utils;
mod visitor;

use log::logger::ILogger;
use parser::parser::Parser;
use utils::cli_args::CLIArgs;

// use crate::log::logger::{STDLogger, STD_LOGGER};

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
        let result = opt_cli_result.unwrap();
        let logger_cell = log::logger::get_std_logger().lock().unwrap();
        let mut logger = logger_cell.borrow_mut();
        logger.fatal(result.1, Some(result.0));
    }

    let input = String::from("{ \"key\": 123.456 }"); // Correct
    let mut parser = Parser::new_copy(&input);
    let root_node_result = parser.parse();

    if root_node_result.is_ok()
    {
        // println!("{:?}", root_node_result.unwrap().get_node_type());
        // println!("{}", root_node_result.unwrap().get_node_type());
        let enum_type = root_node_result.unwrap().get_node_type();
        let logger_cell = log::logger::get_std_logger().lock().unwrap();
        let mut logger = logger_cell.borrow_mut();
        logger.info(enum_type.to_string());
    }

    else
    {
        println!("{}", root_node_result.err().unwrap());
    }

    println!("Done!");
}

