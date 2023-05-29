mod parser;
mod utils;

use parser::lexer::Lexer;

#[allow(unused_imports)]
use crate::parser::token::{EnumTokenType, TokenTrait, TokenBool, TokenDouble, TokenSymbol};

#[macro_use]
extern crate downcast_rs;

fn main()
{
    let input = String::from("{ true .123 }");
    let mut lexer = Lexer::new(&input);

    loop
    {
        let mut do_exit = false;
        let token = lexer.next_token();

        match token
        {

            Ok(the_token) =>
            {
                println!("token type: {:?}", the_token.get_type());

                match the_token.get_type()
                {
                    EnumTokenType::BOOL =>
                    {
                        let token_bool = the_token.downcast_ref::<TokenBool>().unwrap();
                        println!("token_bool: {0}", token_bool.get_value());
                    },
                    EnumTokenType::DOUBLE =>
                    {
                        let token_double = the_token.downcast_ref::<TokenDouble>().unwrap();
                        println!("token_double: {0}", token_double.get_value());
                    },
                    EnumTokenType::SYMBOL =>
                    {
                        let token_symbol = the_token.downcast_ref::<TokenSymbol>().unwrap();
                        println!("token_symbol: {0}", token_symbol.get_symbol());
                    },
                    _ => { println!("EnumTokenType: {:?} is not currently handled/supported", the_token.get_type()); },
                }
            },
            Err(e) => { println!("Error: {0}", e); do_exit = true; },
        }

        if do_exit
        {
            break;
        }
    }

    println!("Done!");
}

