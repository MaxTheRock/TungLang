use crate::Rule;
use pest::iterators::Pairs;

pub fn print_ast(pairs: Pairs<Rule>, indent: usize) -> String {
    let mut result: String = String::new();
    let pairs_iter: Pairs<Rule> = pairs;
    for pair in pairs_iter {
        let rule: Rule = pair.as_rule();
        let span: pest::Span = pair.as_span();
        let text: &str = span.as_str();
        let formatted_text: String = text.replace("\n", "\\n");
        let space: String = "  ".repeat(indent);
        result.push_str(&format!(
            "{space}{rule:?}: '{text}'\n",
            space = space,
            rule = rule,
            text = formatted_text
        ));
        let inner: Pairs<Rule> = pair.into_inner();
        let inner_peek = inner.clone();
        if inner_peek.peek().is_some() {
            let inner_result: String = print_ast(inner, indent + 1);
            result.push_str(&inner_result);
        }
    }
    result
}
