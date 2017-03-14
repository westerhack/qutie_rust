use plugins::plugin::Plugin;
use environment::Environment;
use plugins::plugin::PluginResponse;
use plugins::plugin::PluginResponse::{NoResponse, Response};
use objects::text::{Text, Quote, ESCAPE};

#[derive(Debug)]
pub struct TextPlugin{}

pub static INSTANCE: TextPlugin = TextPlugin{};

impl Plugin for TextPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      let start_quote = if let Some(single_char) = env.stream.peek_char() {
                           if let Some(start_quote) = Quote::from_single_char(single_char) {
                              start_quote
                           } else {
                              return NoResponse
                           }
                        } else {
                           return NoResponse
                        };
      env.stream.next();
      let mut text_acc: String = String::new();
      let mut ret = NoResponse;
      loop {
         let mut was_escaped = false;
         match env.stream.peek_char() {
            Some(single_char) => {
               if let Some(end_quote) = Quote::from_single_char(single_char) {
                  let text = Text::new(text_acc, start_quote, end_quote);
                  ret = Response(Ok(Box::new(text)));
                  break
               } else {
                  text_acc.push(single_char.source_val);
                  was_escaped = single_char.source_val == ESCAPE.source_val;
               }
            }
            None => break
         }
         if was_escaped {
            env.stream.next();
            text_acc.push(env.stream.peek_char().unwrap().source_val);
         }
         env.stream.next(); // this will only occur if a break isnt called
      }
      match ret {
         Response(_) => {
            env.stream.next();
            ret
         }
         _ => ret
      }
   }
}














