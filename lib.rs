// Library version of the small Ada parser used by the Rust server.
// Provides AST types and a parse_ada_to_ast function suitable for unit testing.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeKind {
    Program,
    ProcedureDecl { name: String },
    Identifier { name: String },
    Literal { value: String },
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AstNode {
    pub kind: NodeKind,
    pub children: Vec<AstNode>,
}

/// Very small "parser" that creates an AST from raw text (toy example).
/// The same logic as in the server; kept here for unit testing.
pub fn parse_ada_to_ast(source: &str) -> AstNode {
    let mut children = Vec::new();
    if source.contains("procedure") {
        let proc_name = source
            .split_whitespace()
            .skip_while(|s| *s != "procedure")
            .skip(1)
            .next()
            .unwrap_or("unnamed")
            .trim_matches(|c: char| !c.is_alphanumeric());
        children.push(AstNode {
            kind: NodeKind::ProcedureDecl {
                name: proc_name.to_string(),
            },
            children: vec![AstNode {
                kind: NodeKind::Identifier {
                    name: proc_name.to_string(),
                },
                children: vec![],
            }],
        });
    } else if source.trim().is_empty() {
        children.push(AstNode {
            kind: NodeKind::Unknown,
            children: vec![],
        });
    } else {
        children.push(AstNode {
            kind: NodeKind::Literal {
                value: "<text>".into(),
            },
            children: vec![],
        });
    }

    AstNode {
        kind: NodeKind::Program,
        children,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_returns_unknown_child() {
        let ast = parse_ada_to_ast("");
        assert_eq!(ast.kind, NodeKind::Program);
        assert_eq!(ast.children.len(), 1);
        assert_eq!(ast.children[0].kind, NodeKind::Unknown);
    }

    #[test]
    fn parse_literal_returns_literal_node() {
        let src = "with Ada.Text_IO; use Ada.Text_IO;\n-- some comment\nx : Integer := 0;";
        let ast = parse_ada_to_ast(src);
        assert_eq!(ast.kind, NodeKind::Program);
        assert_eq!(ast.children.len(), 1);
        match &ast.children[0].kind {
            NodeKind::Literal { value } => assert!(value == "<text>"),
            _ => panic!("expected Literal node"),
        }
    }

    #[test]
    fn parse_procedure_detects_procedure_name() {
        let src = "procedure Hello is\nbegin\n null; \nend Hello;";
        let ast = parse_ada_to_ast(src);
        assert_eq!(ast.kind, NodeKind::Program);
        assert_eq!(ast.children.len(), 1);
        match &ast.children[0].kind {
            NodeKind::ProcedureDecl { name } => {
                // name may be "Hello" (or with punctuation trimmed)
                assert!(name.to_lowercase().contains("hello"));
                // Identifier child should match
                assert_eq!(ast.children[0].children.len(), 1);
                match &ast.children[0].children[0].kind {
                    NodeKind::Identifier { name: idname } => {
                        assert!(idname.to_lowercase().contains("hello"));
                    }
                    _ => panic!("expected Identifier child"),
                }
            }
            _ => panic!("expected ProcedureDecl node"),
        }
    }

    #[test]
    fn serde_roundtrip() {
        let src = "procedure P is begin null; end P;";
        let ast = parse_ada_to_ast(src);
        let s = serde_json::to_string(&ast).expect("serialize");
        let parsed: AstNode = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(ast, parsed);
    }
}