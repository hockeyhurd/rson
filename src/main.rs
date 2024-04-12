mod log;
mod parser;
mod rnodes;
mod utils;
mod visitor;

#[allow(unused_imports)]
use log::logger::ILogger;
use parser::parser::Parser;

extern crate chrono;

#[macro_use]
extern crate downcast_rs;

fn main()
{
    // let input = String::from("{ true .123 \\\"Hello, world!\\\" }");
    // let input = String::from("{ \"\\\"Hi\\\"\" }"); // Correct
    let input = String::from("{ \"key\": 123.456 }"); // Correct
    let mut parser = Parser::new_copy(&input);
    let root_node_result = parser.parse();

    if root_node_result.is_ok()
    {
        // println!("{:?}", root_node_result.unwrap().get_node_type());
        println!("{}", root_node_result.unwrap().get_node_type());
    }

    else
    {
        println!("{}", root_node_result.err().unwrap());
    }

    println!("Done!");
}

