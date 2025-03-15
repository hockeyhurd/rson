use crate::parser::parser::Parser;
use crate::rnodes::rnode::RNode;

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

pub struct RsonReader
{
    parser: RefCell<Parser>,
    #[allow(dead_code)]
    pub file_path: Option<String>,
}

impl RsonReader
{
    pub fn from_file(path: &String, stringify: bool) -> Self
    {
        let input = std::fs::read_to_string(&path).expect("Failed to read input file");
        Self { parser: RefCell::new(Parser::new_move(input, stringify)), file_path: Some(path.clone()) }
    }

    pub fn from_literal(input: &String, stringify: bool) -> Self
    {
        Self { parser: RefCell::new(Parser::new_copy(&input, stringify)), file_path: None }
    }

    pub fn from_stdin(stringify: bool) -> Self
    {
        let input = std::io::read_to_string(std::io::stdin()).expect("Failed to read from stdin");
        Self { parser: RefCell::new(Parser::new_move(input, stringify)), file_path: None }
    }

    pub fn parse(&self) -> Result<Rc<dyn RNode>, String>
    {
        let mut parser: RefMut<Parser> = self.parser.borrow_mut();
        parser.parse()
    }
}

