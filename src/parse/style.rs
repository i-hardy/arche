use std::collections::HashMap;

use crate::parse::{
    dom::{ElementData, Node, NodeType::{Element,Text}},
    cssom::{Rule, Selector, SimpleSelector, Specificity, StyleSheet, Value},
};

type PropertyMap = HashMap<String, Value>;

#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

type MatchedRule<'a> = (Specificity, &'a Rule);

pub fn style_tree<'a>(node: &'a Node, stylesheet: &'a StyleSheet, parent_node: Option<&'a Node>) -> StyledNode<'a> {
    StyledNode {
        node,
        specified_values: determine_specified_values(node, stylesheet, parent_node),
        children: node
            .children
            .iter()
            .map(|child| style_tree(child, stylesheet, Some(node)))
            .collect(),
    }
}

fn determine_specified_values<'a>(node: &'a Node, stylesheet: &'a StyleSheet, parent_node: Option<&'a Node>) -> PropertyMap {
	match node.node_type {
			Element(ref element) => specified_values(element, stylesheet),
			Text(_) => match parent_node {
				Some(node) => match node.node_type {
					Element(ref element) => specified_values(element, stylesheet),
					_ => HashMap::new(),
				}
				None => HashMap::new()
			},
			_ => HashMap::new(),
	}
}

fn specified_values(element: &ElementData, stylesheet: &StyleSheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(element, stylesheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}

fn matching_rules<'a>(element: &ElementData, stylesheet: &'a StyleSheet) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(element, rule))
        .collect()
}

fn match_rule<'a>(element: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches(element, *selector))
        .map(|selector| (selector.specificity(), rule))
}

fn matches(element: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(element, simple_selector),
    }
}

fn matches_simple_selector(element: &ElementData, selector: &SimpleSelector) -> bool {
    if selector
        .tag_name
        .iter()
        .any(|name| element.tag_name != *name)
    {
        return false;
    }

    if selector.id.iter().any(|id| element.id() != Some(id)) {
        return false;
    }

    let classes = element.classes();
    if selector
        .class
        .iter()
        .any(|class| !classes.contains(&**class))
    {
        return false;
    }

    true
}
