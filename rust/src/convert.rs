use pulldown_cmark::{Event, Options, Parser};

use crate::errors::ConvertError;
use crate::qase::{Case, Suite};

#[derive(Debug)]
struct TextNode {
    id: i32,
    text: String,
    nesting_level: usize,
    children: Vec<TextNode>,
}

/// Walks the markdown AST and creates a flat array
/// of Case nodes along with the nesting level at which they
/// occur.
fn first_pass(markdown_input: &str, parsed_texts: &mut Vec<TextNode>) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    let mut nesting_level = 0;
    let event_iterator = Parser::new_ext(markdown_input, options);
    for event in event_iterator {
        match event {
            Event::Start(_) => {
                nesting_level += 1;
            }
            Event::Text(text) => {
                parsed_texts.push(TextNode {
                    id: parsed_texts.len() as i32 + 1,
                    text: text.to_string().trim().to_string(),
                    nesting_level,
                    children: vec![],
                });
            }
            Event::End(_) => {
                nesting_level -= 1;
            }
            _ => {}
        }
    }
}

/// In the result of the first pass, we have a flat array of nodes along with
/// the nesting levels at which they occur. In this pass, we walk the array
/// and create a tree, with nodes added as children when their nesting level
/// is higher than that of a prev node
fn second_pass(text_nodes: &Vec<TextNode>, next_index: &mut usize) -> TextNode {
    let current_node = &text_nodes[*next_index];
    let mut children: Vec<TextNode> = vec![];
    loop {
        if *next_index + 1 >= text_nodes.len() {
            break;
        }
        let next_node_level = &text_nodes[*next_index + 1].nesting_level;
        if next_node_level <= &current_node.nesting_level {
            // This means that the upcoming node is
            // not a child of this one
            break;
        }
        *next_index += 1;
        children.push(second_pass(text_nodes, next_index));
    }
    return TextNode {
        id: current_node.id,
        text: current_node.text.clone(),
        nesting_level: current_node.nesting_level,
        children,
    };
}

/// In this final pass, we create the structures specific to Qase
/// i.e. a combination of Case and Suite structs nested as necessary
fn third_pass(node: &TextNode, is_root: bool) -> (Option<Suite>, Option<Case>) {
    if !is_root && node.children.len() == 0 {
        let case = Case {
            id: node.id,
            title: node.text.clone(),
            description: Some(String::from("")),
            preconditions: Some(String::from("")),
            postconditions: Some(String::from("")),
            priority: String::from("medium"),
            severity: String::from("undefined"),
            behavior: String::from("undefined"),
            test_type: String::from("other"),
            layer: String::from("unknown"),
            is_flaky: String::from("no"),
            automation: String::from("is-not-automated"),
            status: String::from("actual"),
            milestone: None,
            custom_fields: Vec::new(),
            steps: Vec::new(),
        };
        return (None, Some(case));
    }
    let mut child_cases: Vec<Case> = Vec::new();
    let mut child_suites: Vec<Suite> = Vec::new();

    for c in &node.children {
        let (child_suite, child_case) = third_pass(c, false);
        match child_case {
            Some(case) => {
                child_cases.push(case);
            }
            None => {}
        }
        match child_suite {
            Some(suite) => {
                child_suites.push(suite);
            }
            None => {}
        }
    }

    let suite = Suite {
        id: Some(node.id),
        title: Some(node.text.clone()),
        description: Some(String::from("")),
        preconditions: None,
        cases: child_cases,
        suites: child_suites,
    };
    return (Some(suite), None);
}

pub fn convert_md_to_qase(suite_header: &str, markdown_input: &str) -> Result<Suite, ConvertError> {
    let mut parsed_cases = Vec::new();
    parsed_cases.push(TextNode {
        id: 0,
        text: String::from(suite_header),
        nesting_level: 0,
        children: vec![],
    });
    first_pass(markdown_input, &mut parsed_cases);

    let mut next_index: usize = 0;
    let parsed_root = second_pass(&parsed_cases, &mut next_index);

    let (result, _) = third_pass(&parsed_root, true);
    return match result {
        Some(suite) => Ok(suite),
        None => Err(ConvertError::SuiteMissingError),
    };
}
