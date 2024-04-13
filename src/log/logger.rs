use std::cell::RefCell;
use std::collections::HashMap;
use std::ptr::addr_of;
use std::sync::Mutex;
use chrono;

#[derive(Clone, Copy)]
pub enum EnumLogLevel
{
    #[allow(dead_code)]
    FATAL, ERROR, WARN, INFO, DEBUG
}

pub trait ILogger
{
    fn debug(&mut self, msg: String);
    fn error(&mut self, msg: String);
    fn fatal(&mut self, msg: String, opt_error_code: Option<i32>);
    fn info(&mut self, msg: String);
    fn warn(&mut self, msg: String);
}

pub struct STDLogger
{
    cur_log_level: EnumLogLevel,
}

static STD_LOGGER: Mutex<RefCell<STDLogger>> = Mutex::new(RefCell::new(STDLogger { cur_log_level: EnumLogLevel::WARN }));

impl EnumLogLevel
{
    pub fn ordinal(self) -> u32
    {
        self as u32
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String
    {
        match self
        {
            EnumLogLevel::DEBUG => { String::from("DEBUG") },
            EnumLogLevel::ERROR => { String::from("ERROR") },
            EnumLogLevel::FATAL => { String::from("FATAL") },
            EnumLogLevel::INFO => { String::from("INFO") },
            EnumLogLevel::WARN => { String::from("WARN") },
        }
    }
}

pub fn get_log_level_from_string(string: &String) -> Result<EnumLogLevel, String>
{
    static mut LOOKUP_TABLE: Option<HashMap<String, EnumLogLevel>> = None;

    unsafe
    {
        if LOOKUP_TABLE.is_none()
        {
            let mut table = HashMap::<String, EnumLogLevel>::new();
            table.insert(String::from("DEBUG"), EnumLogLevel::DEBUG);
            table.insert(String::from("ERROR"), EnumLogLevel::ERROR);
            table.insert(String::from("FATAL"), EnumLogLevel::FATAL);
            table.insert(String::from("INFO"), EnumLogLevel::INFO);
            table.insert(String::from("WARN"), EnumLogLevel::WARN);

            LOOKUP_TABLE = Some(table);
        }

        let table_ptr = addr_of!(LOOKUP_TABLE);
        let opt_table = table_ptr.as_ref().unwrap();

        if let Some(table) = opt_table
        {
            let result = table.get(string);

            if result.is_some()
            {
                return Ok(*result.unwrap());
            }
        }
    }

    return Err(String::from("String is not a EnumLogLevel"));
}

impl ILogger for STDLogger
{
    fn debug(&mut self, msg: String)
    {
        if !self.can_log(EnumLogLevel::DEBUG)
        {
            return;
        }

        let timestamp = chrono::offset::Utc::now();
        println!("[DEBUG] {0}: {1}", timestamp, msg);
    }

    fn error(&mut self, msg: String)
    {
        if !self.can_log(EnumLogLevel::ERROR)
        {
            return;
        }

        let timestamp = chrono::offset::Utc::now();
        println!("[ERROR] {0}: {1}", timestamp, msg);
    }

    fn fatal(&mut self, msg: String, opt_error_code: Option<i32>)
    {
        if !self.can_log(EnumLogLevel::FATAL)
        {
            return;
        }

        let timestamp = chrono::offset::Utc::now();
        println!("[FATAL] {0}: {1}", timestamp, msg);

        let mut error_code: i32 = -1;

        if opt_error_code.is_some()
        {
            error_code = opt_error_code.unwrap();
        }

        std::process::exit(error_code);
    }

    fn info(&mut self, msg: String)
    {
        if !self.can_log(EnumLogLevel::INFO)
        {
            return;
        }

        let timestamp = chrono::offset::Utc::now();
        println!("[INFO] {0}: {1}", timestamp, msg);
    }

    fn warn(&mut self, msg: String)
    {
        if !self.can_log(EnumLogLevel::WARN)
        {
            return;
        }

        let timestamp = chrono::offset::Utc::now();
        println!("[WARN] {0}: {1}", timestamp, msg);
    }
}

impl STDLogger
{
    fn can_log(&self, log_level: EnumLogLevel) -> bool
    {
        return self.cur_log_level.clone().ordinal() >= log_level.clone().ordinal();
    }

    pub fn set_log_level(&mut self, log_level: EnumLogLevel)
    {
        self.cur_log_level = log_level;
    }
}

#[allow(dead_code)]
pub fn get_std_logger() -> &'static Mutex<RefCell<STDLogger>>
{
    return &STD_LOGGER;
}

