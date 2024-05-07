use crate::log::logger::{ILogger, get_std_logger};
#[allow(unused_imports)]
use crate::rnodes::rnode::{EnumNodeType, RNode};
use crate::rnodes::rnode_array::RNodeArray;
use crate::rnodes::rnode_bool::RNodeBool;
use crate::rnodes::rnode_double::RNodeDouble;
use crate::rnodes::rnode_null::RNodeNull;
use crate::rnodes::rnode_object::RNodeObject;
use crate::rnodes::rnode_string::RNodeString;
use crate::utils::string_utils::StringBuilder;
use crate::visitor::visitor::Visitor;

use std::cell::{Cell, RefCell, RefMut};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::io::Stdout;
use std::rc::Rc;

#[allow(dead_code)]
pub struct RsonWriter
{
    writer: Option<BufWriter<File>>,
    stdout: Option<BufWriter<Stdout>>,
    indent: u32,
    cur_indent: Cell<u32>,
    pub builder: RefCell<StringBuilder>,
}

impl RsonWriter
{
    #[allow(dead_code)]
    pub fn new_file(path: &String, indent: u32) -> std::io::Result<Self>
    {
        let file_opt = BufWriter::new(OpenOptions::new().write(true).create(true).open(&path)?);
        let writer = RsonWriter
           {
               writer: Some(file_opt), stdout: None,
               indent, cur_indent: Cell::new(0),
               builder: RefCell::new(StringBuilder::new(4096)),
           };

        Ok(writer)
    }

    #[allow(dead_code)]
    pub fn new_stdout(indent: u32) -> std::io::Result<Self>
    {
        let handle = std::io::stdout();
        let file_opt = BufWriter::new(handle);
        let writer = RsonWriter
           {
               writer: None, stdout: Some(file_opt),
               indent, cur_indent: Cell::new(0),
               builder: RefCell::new(StringBuilder::new(4096)),
           };

        Ok(writer)
    }

    fn decrement_indent(&self)
    {
        let cur_indent = self.cur_indent.get();

        if cur_indent >= self.indent
        {
            self.cur_indent.set(cur_indent - self.indent);
        }
    }

    fn increment_indent(&self)
    {
        let cur_indent = self.cur_indent.get();
        self.cur_indent.set(cur_indent + self.indent);
    }

    fn insert_indent(&self)
    {
        let cur_indent = self.cur_indent.get();
        let mut builder: RefMut<StringBuilder> = self.builder.borrow_mut();

        for _ in 0..cur_indent
        {
            builder.append_char(' ');
        }
    }

    #[allow(dead_code)]
    pub fn flush(&mut self) -> std::io::Result<bool>
    {
        let mut builder: RefMut<StringBuilder> = self.builder.borrow_mut();

        // Check if there is anything to write.
        if builder.empty()
        {
            return Ok(false);
        }

        // Make sure we always end with a second new line
        if let Some(ch) = builder.get(builder.len() - 1)
        {
            const NEWLINE: char = '\n';

            if *ch != NEWLINE
            {
                builder.append_str("\n\n");
            }

            else if let Some(ch) = builder.get(builder.len() - 2)
            {
                if *ch != NEWLINE
                {
                    builder.append_char(NEWLINE);
                }
            }
        }

        let output = builder.to_string();

        if let Some(writer) = &mut self.writer
        {
            let mut result = writer.write_all(output.as_bytes());

            if result.is_ok()
            {
                result = writer.flush();
            }

            return Ok(result.is_ok());
        }

        if let Some(stdout) = &mut self.stdout
        {
            let mut result = stdout.write_all(output.as_bytes());

            if result.is_ok()
            {
                result = stdout.flush();
            }

            return Ok(result.is_ok());
        }

        return Ok(false);
    }

    #[allow(dead_code)]
    pub fn write(&mut self, node: Rc<dyn RNode>) -> std::io::Result<bool>
    {
        match node.get_node_type()
        {
            EnumNodeType::ARRAY | EnumNodeType::DOUBLE | EnumNodeType::OBJECT =>
            {
                node.accept(self);
            },
            _ => { return Ok(false); }
        }

        Ok(true)
    }
}

impl Visitor for RsonWriter
{
    fn visit_array(&self, node: &RNodeArray)
    {
        {
            let mut builder: RefMut<StringBuilder> = self.builder.borrow_mut();
            builder.append_char('[');
        }

        let len = node.len();

        for i in 0..len
        {
            let opt_subnode: Option<Rc<dyn RNode>> = node.get(i);

            match opt_subnode
            {
                Some(subnode) =>
                {
                    subnode.clone().accept(self);

                    if i + 1 < len
                    {
                        let mut builder: RefMut<StringBuilder> = self.builder.borrow_mut();
                        builder.append_str(", ");
                    }
                },
                None =>
                {
                    let logger_cell = get_std_logger().lock().unwrap();
                    let mut logger = logger_cell.borrow_mut();
                    logger.fatal(String::from("Critical error while retrieving a node from an RNodeArray"), Some(-3));
                }
            }
        }

        {
            let mut builder: RefMut<StringBuilder> = self.builder.borrow_mut();
            builder.append_char(']');
        }
    }

    fn visit_bool(&self, node: &RNodeBool)
    {
        let mut builder: RefMut<StringBuilder> = self.builder.borrow_mut();

        if node.value
        {
            builder.append_str("true");
        }

        else
        {
            builder.append_str("false");
        }
    }

    fn visit_double(&self, node: &RNodeDouble)
    {
        let mut builder: RefMut<StringBuilder> = self.builder.borrow_mut();
        builder.append_string(&node.value.to_string());
    }

    fn visit_null(&self, _node: &RNodeNull)
    {
        let mut builder: RefMut<StringBuilder> = self.builder.borrow_mut();
        builder.append_str("null");
    }

    fn visit_object(&self, node: &RNodeObject)
    {
        self.insert_indent();
        self.increment_indent();

        {
            let mut builder: RefMut<StringBuilder> = self.builder.borrow_mut();
            builder.append_str("{\n");
        }

        {
            let objmap = node.get_map();
            let len: usize = objmap.len();
            let mut count: usize = 0;

            for (key, value) in objmap
            {
                self.insert_indent();

                {
                    let mut builder: RefMut<StringBuilder> = self.builder.borrow_mut();
                    builder.append_char('"');
                    builder.append_string(&key);
                    builder.append_str("\": ");
                }

                value.accept(self);
                count += 1;

                {
                    let mut builder: RefMut<StringBuilder> = self.builder.borrow_mut();

                    if count < len
                    {
                        builder.append_str(",\n");
                    }

                    else
                    {
                        builder.append_char('\n');
                    }
                }
            }
        }

        {
            let mut builder: RefMut<StringBuilder> = self.builder.borrow_mut();
            builder.append_str("}\n");
        }

        self.decrement_indent();
    }

    fn visit_string(&self, node: &RNodeString)
    {
        let mut builder: RefMut<StringBuilder> = self.builder.borrow_mut();
        builder.append_char('"');
        builder.append_string(&node.get_value());
        builder.append_char('"');
    }
}

#[cfg(test)]
mod tests
{
    use crate::rnodes::rnode::RNode;
    use crate::rnodes::rnode_array::RNodeArray;
    use crate::rnodes::rnode_bool::RNodeBool;
    use crate::rnodes::rnode_double::RNodeDouble;
    use crate::rnodes::rnode_string::RNodeString;
    use super::RsonWriter;

    use std::rc::Rc;

    const DEFAULT_INDENT: u32 = 4;

    #[test]
    fn create_writer()
    {
        let path = String::from("test.json");
        let writer_result = RsonWriter::new_file(&path, DEFAULT_INDENT);
        assert!(writer_result.is_ok());

        let writer = writer_result.unwrap();
        assert_eq!(writer.indent, DEFAULT_INDENT);

        // Clean-up after our test.
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn increment_decrement_indent()
    {
        let writer_result = RsonWriter::new_stdout(DEFAULT_INDENT);
        assert!(writer_result.is_ok());

        let writer = writer_result.unwrap();
        assert_eq!(writer.indent, DEFAULT_INDENT);
        assert_eq!(writer.cur_indent.get(), 0);

        writer.increment_indent();
        assert_eq!(writer.cur_indent.get(), DEFAULT_INDENT);

        writer.decrement_indent();
        assert_eq!(writer.cur_indent.get(), 0);
    }

    #[test]
    fn write_array()
    {
        let writer_result = RsonWriter::new_stdout(DEFAULT_INDENT);
        assert!(writer_result.is_ok());

        let mut vec: Vec<Rc<dyn RNode>> = Vec::new();
        vec.push(Rc::new(RNodeDouble::new(1.0)));
        vec.push(Rc::new(RNodeString::new_move(String::from("Hello, world!"))));
        vec.push(Rc::new(RNodeBool::new(false)));

        let node_array = Rc::new(RNodeArray::new(vec));
        let mut writer = writer_result.unwrap();
        let result = writer.write(node_array.clone());
        assert!(result.is_ok());

        let result = writer.flush();
        assert!(result.is_ok());
    }

    #[test]
    fn write_bool()
    {
        let writer_result = RsonWriter::new_stdout(DEFAULT_INDENT);
        assert!(writer_result.is_ok());

        let node_bool = Rc::new(RNodeBool::new(true));
        let mut writer = writer_result.unwrap();
        let result = writer.write(node_bool.clone());
        assert!(result.is_ok());

        let result = writer.flush();
        assert!(result.is_ok());
    }

    #[test]
    fn write_double()
    {
        let writer_result = RsonWriter::new_stdout(DEFAULT_INDENT);
        assert!(writer_result.is_ok());

        let node_double = Rc::new(RNodeDouble::new(123.45E10));
        let mut writer = writer_result.unwrap();
        let result = writer.write(node_double.clone());
        assert!(result.is_ok());

        let result = writer.flush();
        assert!(result.is_ok());
    }
}

