use kaitai_rs::core::ast::Node;
use kaitai_rs::core::ast::NodeRef;
use kaitai_rs::core::ast::AST;

// This file contains unit tests for the Node and AST structs implemented in the kaitai_rs library.
// The tests cover various use cases by creating nodes, adding children, setting data, and traversing
// the AST in depth-first order. Each test function focuses on a specific aspect of the Node and AST
// functionality and provides sets of expected and actual results for thorough validation.

#[test]
fn test_node_new() {
    // Test creating a new Node with no parent, children, or data
    let node: NodeRef<i32> = Node::new();
    assert!(node.borrow().get_parent().is_none());
    assert_eq!(node.borrow().get_children().len(), 0);
    assert!(node.borrow().get_data().is_none());
}

#[test]
fn test_node_set_data() {
    // Test setting data on a Node
    let node: NodeRef<i32> = Node::new();
    node.borrow_mut().set_data(42);
    assert_eq!(node.borrow().get_data(), Some(&42));
}

#[test]
fn test_node_set_parent() {
    // Test setting the parent of a Node
    let parent: NodeRef<i32> = Node::new();
    let child: NodeRef<i32> = Node::new();
    child.borrow_mut().set_parent(parent.clone());
    assert_eq!(child.borrow().get_parent(), Some(&parent));
}

#[test]
fn test_node_add_child() {
    // Test adding children to a Node
    let parent: NodeRef<i32> = Node::new();
    let child1: NodeRef<i32> = Node::new();
    let child2: NodeRef<i32> = Node::new();
    parent.borrow_mut().add_child(child1.clone());
    parent.borrow_mut().add_child(child2.clone());
    assert_eq!(parent.borrow().get_children().len(), 2);
    assert_eq!(parent.borrow().get_children()[0], child1);
    assert_eq!(parent.borrow().get_children()[1], child2);
}

#[test]
fn test_ast_new() {
    // Test creating a new AST with an empty root Node
    let ast = AST::<i32>::new();
    let root = ast.get_root().clone();
    assert_eq!(root.borrow().get_children().len(), 0);
    assert_eq!(root.borrow().get_data(), None);
}

#[test]
fn test_ast_traverse() {
    // Test traversing an AST in depth-first order
    let root: NodeRef<i32> = Node::new();
    let child1: NodeRef<i32> = Node::new();
    let child2: NodeRef<i32> = Node::new();
    root.borrow_mut().add_child(child1.clone());
    root.borrow_mut().add_child(child2.clone());
    let mut ast = AST::new();
    ast.set_root(root.clone());
    let mut nodes = vec![];
    ast.traverse(|node| nodes.push(node.clone()));
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[0], root);
    assert_eq!(nodes[1], child1);
    assert_eq!(nodes[2], child2);
}
