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

// @@@ test only
use crate::rnodes::rnode::RNode;
use std::rc::Rc;

fn test_write(node: Rc<dyn RNode>)
{
    let path = String::from("test.json");
    let writer_result = RsonWriter::new(&path, 4);

    match writer_result
    {
        Ok(mut writer) =>
        {
            let _ = writer.write(node);
            let _ = writer.flush();
        },
        Err(_e) => {}
    }
}

// @@@ test only

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

    let rson_reader: RsonReader;

    match cli_args.input_file
    {
        Some(input_file) => { rson_reader = RsonReader::from_file(&input_file); },
        None => { rson_reader = RsonReader::from_stdin() }
    }

    let root_node_result = rson_reader.parse();

    match root_node_result
    {
        Ok(root_node) =>
        {
            let enum_type = root_node.get_node_type();
            let logger_cell = log::logger::get_std_logger().lock().unwrap();
            let mut logger = logger_cell.borrow_mut();
            logger.info(enum_type.to_string());
            test_write(root_node.clone());
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

