use crate::log::logger::{EnumLogLevel, get_log_level_from_string, get_std_logger, ILogger};

pub struct CLIArgs
{
    input_file: Option<String>,
    log_level: EnumLogLevel,
}

impl CLIArgs
{
    pub fn new() -> Self
    {
        Self { input_file: None, log_level: EnumLogLevel::WARN }
    }

    pub fn parse(&mut self, args: &Vec<String>) -> Option<(i32, String)>
    {
        let arg_count = args.len();

        if arg_count <= 1
        {
            return Some((-1, String::from("Expected at least one argument")));
        }

        let mut skip_next = false;

        for i in 1..arg_count
        {
            if skip_next
            {
                skip_next = false;
                continue;
            }

            let arg = args.get(i).expect("Failed to unwrap argument");

            if arg == "--input"
            {
                let opt_next_arg = args.get(i + 1);

                if opt_next_arg.is_none()
                {
                    return Some((-1, String::from("Expected an input file after the argument '--input'")));
                }

                self.input_file = Some(opt_next_arg.unwrap().clone());
            }

            else if arg == "--log-level"
            {
                let opt_next_arg = args.get(i + 1);

                if opt_next_arg.is_none()
                {
                    return Some((-1, String::from("Expected an input file after the argument '--log-level'")));
                }

                let result_log_level = get_log_level_from_string(opt_next_arg.unwrap());
                let logger_cell = get_std_logger().lock().unwrap();
                let mut logger = logger_cell.borrow_mut();

                match result_log_level
                {
                    Ok(log_level) =>
                    {
                        self.log_level = log_level;
                        logger.set_log_level(log_level);
                    },
                    Err(msg) =>
                    {
                        logger.warn(msg);
                    },
                }
            }
        }

        // return Some((-1, String::from("Failed to parse input arguments.")));
        return None;
    }
}

