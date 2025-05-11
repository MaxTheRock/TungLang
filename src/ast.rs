use pest::iterators::Pairs;
use crate::Rule;

pub fn print_ast(parsed: &Pairs<Rule>, indent: usize) {
    for pair in parsed.clone() {
        let rule: String = format!("{:?}", pair.as_rule());
        let span = pair.as_span();
        let text: String = span.as_str().replace("\n", "\\n");
        println!("{space}{rule}: '{text}'", space = "  ".repeat(indent), rule = rule, text = text);
        let inner = pair.clone().into_inner();
        if inner.clone().count() > 0 {
            print_ast(&inner, indent + 1);
        }
    }
}
