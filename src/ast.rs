use pest::iterators::Pairs;
use crate::Rule;

 pub fn print_ast(parsed: &Pairs<Rule>, indent: usize) -> String {
     let mut result = String::new();
     for pair in parsed.clone() {
         let rule: String = format!("{:?}", pair.as_rule());
         let span = pair.as_span();
         let text: String = span.as_str().replace("\n", "\\n");
         result.push_str(&format!("{space}{rule}: '{text}'\n", 
                      space = "  ".repeat(indent), rule = rule, text = text));
         
         let inner = pair.into_inner();
         if !inner.clone().peek().is_none() {
             result.push_str(&print_ast(&inner, indent + 1));
         }
     }
     result
 }
