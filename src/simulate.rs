use crate::{
    ast::{Node, NodeType},
    symbol::StringList,
};

pub fn simulate(ast: &Box<Node>, string_list: &StringList) {
    for statement in &ast.children {
        match statement.node_type {
            NodeType::PrintStatement => {
                let expression = &statement.children[0];
                match expression.node_type {
                    NodeType::Expression => {
                        let token = &expression.token.as_ref().unwrap();
                        match token.token_type() {
                            crate::lexer::TokenType::StringListIndex(index) => {
                                let string = string_list.get(*index);
                                println!("{}", string);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
