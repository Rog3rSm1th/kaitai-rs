use kaitai_rs::core::ast::Node;
use kaitai_rs::core::ast::NodeRef;
use kaitai_rs::core::ast::NodeType;
use kaitai_rs::core::ast::AST;

// This file contains unit tests for the Node and AST structs implemented in the kaitai_rs library.
// The tests cover various use cases by creating nodes, adding children, setting data, and traversing
// the AST in depth-first order. Each test function focuses on a specific aspect of the Node and AST
// functionality and provides sets of expected and actual results for thorough validation.

#[test]
// Test creating a new Node with no parent, children, or data
fn test_node_new() {
    let node: NodeRef = Node::new(Some("test_node".to_string()));
    assert!(node.borrow().get_parent().is_none());
    assert_eq!(node.borrow().get_children().len(), 0);
    assert!(node.borrow().get_data().is_none());
}

#[test]
// Test setting data on a Node
fn test_node_set_data() {
    let node: NodeRef = Node::new(Some("test_node".to_string()));
    node.borrow_mut().set_data(vec![42]);
    assert_eq!(node.borrow().get_data(), Some(&vec![42]));
}

#[test]
// Test the reconstruction of a parent node's data from its children nodes data
fn test_node_get_data_from_children() {
    let parent: NodeRef = Node::new(Some("test_node".to_string()));
    let child1: NodeRef = Node::new(Some("test_child1_node".to_string()));
    let child2: NodeRef = Node::new(Some("test_child2_node".to_string()));
    // We also add grandchildren to test the recursive property of get_data_from_children method
    let grandchild1: NodeRef = Node::new(Some("test_grandchild1_node".to_string()));
    let grandchild2: NodeRef = Node::new(Some("test_grandchild2_node".to_string()));

    child1.borrow_mut().add_child(grandchild1.clone());
    child2.borrow_mut().add_child(grandchild2.clone());
    parent.borrow_mut().add_child(child1.clone());
    parent.borrow_mut().add_child(child2.clone());

    child1.borrow_mut().set_data(vec![1]);
    child2.borrow_mut().set_data(vec![2]);
    grandchild1.borrow_mut().set_data(vec![3]);
    grandchild2.borrow_mut().set_data(vec![4]);

    let data = parent.borrow().get_data_from_children();
    assert_eq!(data, vec![vec![1], vec![3], vec![2], vec![4]]);
}

#[test]
// Test setting the parent of a Node
fn test_node_set_parent() {
    let parent: NodeRef = Node::new(Some("test_node".to_string()));
    let child: NodeRef = Node::new(Some("test_child_node".to_string()));
    child.borrow_mut().set_parent(parent.clone());
    assert_eq!(child.borrow().get_parent(), Some(&parent));
}

#[test]
// Test adding children to a Node
fn test_node_add_child() {
    let parent: NodeRef = Node::new(Some("test_node".to_string()));
    let child1: NodeRef = Node::new(Some("test_child1_node".to_string()));
    let child2: NodeRef = Node::new(Some("test_child2_node".to_string()));
    parent.borrow_mut().add_child(child1.clone());
    parent.borrow_mut().add_child(child2.clone());
    assert_eq!(parent.borrow().get_children().len(), 2);
    assert_eq!(parent.borrow().get_children()[0], child1);
    assert_eq!(parent.borrow().get_children()[1], child2);
}

#[test]
// Test creating a new AST with an empty root Node
fn test_ast_new() {
    let ast = AST::new();
    let root = ast.get_root().clone();
    assert_eq!(root.borrow().get_children().len(), 0);
    assert_eq!(root.borrow().get_data(), None);
}

#[test]
// Test traversing an AST in depth-first order
fn test_ast_traverse() {
    let root: NodeRef = Node::new(Some("test_node".to_string()));
    let child1: NodeRef = Node::new(Some("test_child1_node".to_string()));
    let child2: NodeRef = Node::new(Some("test_child2_node".to_string()));
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

#[test]
// Test getting a node by ID
fn test_ast_get_node_by_id() {
    let root: NodeRef = Node::new(Some("root".to_string()));
    let child1: NodeRef = Node::new(Some("child1".to_string()));
    let child2: NodeRef = Node::new(Some("child2".to_string()));
    let grandchild1: NodeRef = Node::new(Some("grandchild1".to_string()));
    let grandchild2: NodeRef = Node::new(Some("grandchild2".to_string()));

    child1.borrow_mut().add_child(grandchild1.clone());
    child2.borrow_mut().add_child(grandchild2.clone());
    root.borrow_mut().add_child(child1.clone());
    root.borrow_mut().add_child(child2.clone());

    let mut ast = AST::new();
    ast.set_root(root.clone());

    assert_eq!(
        ast.get_node_by_id("root"),
        Some(root.clone().borrow().clone())
    );
    assert_eq!(
        ast.get_node_by_id("child1"),
        Some(child1.clone().borrow().clone())
    );
    assert_eq!(
        ast.get_node_by_id("child2"),
        Some(child2.clone().borrow().clone())
    );
    assert_eq!(
        ast.get_node_by_id("grandchild1"),
        Some(grandchild1.clone().borrow().clone())
    );
    assert_eq!(
        ast.get_node_by_id("grandchild2"),
        Some(grandchild2.clone().borrow().clone())
    );
    assert_eq!(ast.get_node_by_id("nonexistent"), None);
}

#[test]
// Test setting and getting the node type
fn test_node_type() {
    let node: NodeRef = Node::new(Some("test_node".to_string()));

    // Set node type to Integer
    node.borrow_mut().set_node_type(NodeType::Integer);
    assert_eq!(node.borrow().get_node_type(), Some(&NodeType::Integer));

    // Set node type to String
    node.borrow_mut().set_node_type(NodeType::String);
    assert_eq!(node.borrow().get_node_type(), Some(&NodeType::String));

    // Set node type to Array
    node.borrow_mut().set_node_type(NodeType::Array);
    assert_eq!(node.borrow().get_node_type(), Some(&NodeType::Array));
}
