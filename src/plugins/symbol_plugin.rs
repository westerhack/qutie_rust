use env::Environment;
use std::rc::Rc;
use parser::Parser;
use objects::universe::Universe;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::symbol::Symbol;
use result::ObjError;

#[derive(Debug)]
pub struct SymbolPlugin;

pub static INSTANCE: SymbolPlugin = SymbolPlugin{};

impl Plugin for SymbolPlugin {

   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match peek_char!(env, EndOfFile => '0') {
         e if e.is_alphabetic() => {},
         e if e == '_' => {},
         _ => return PluginResponse::NoResponse
      };

      let mut symbol_acc: String = String::new();

      loop {
         match env.stream.peek_char() {
            Ok(peeked_struct) => {
               let peeked_char = peeked_struct.char_val;
               if peeked_char.is_alphanumeric() || peeked_char == '_' {
                  symbol_acc.push(peeked_char);
               } else {
                  break
               }
            },
            Err(ObjError::EndOfFile) => break,
            Err(err) => panic!("Don't know how to deal with error: {:?}", err)
         }
         let _next_char = env.stream.next(); // this will only occur if a break isnt called
      }

      if symbol_acc.is_empty() {
         PluginResponse::NoResponse
      } else {
         ok_rc!(RESP; Symbol::new(symbol_acc))
      }
   }

}














