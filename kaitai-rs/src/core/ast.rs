use std::cell::RefCell;
use std::rc::Rc;

pub type NodeRef<T> = Rc<RefCell<Node<T>>>;

/// A struct representing a node in an Abstract Syntax Tree (AST)
#[derive(Debug)]
pub struct Node<T> {
    /// The parent of this node in the AST, if any
    parent: Option<NodeRef<T>>,

    /// The children of this node in the AST, if any
    children: Vec<NodeRef<T>>,

    /// The data associated with this node in the AST, if any
    data: Option<T>,
}

impl<T> Node<T> {
    /// Creates a new `Node` with no parent, no children, and no data
    pub fn new() -> NodeRef<T> {
        Rc::new(RefCell::new(Node {
            parent: None,
            children: Vec::new(),
            data: None,
        }))
    }

    /// Sets the data associated with this node in the AST
    pub fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }

    /// Gets the data associated with this node in the AST, if any
    pub fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// Sets the parent of this node in the AST
    pub fn set_parent(&mut self, parent: NodeRef<T>) {
        self.parent = Some(parent);
    }

    /// Gets the parent of this node in the AST, if any
    pub fn get_parent(&self) -> Option<&NodeRef<T>> {
        self.parent.as_ref()
    }

    /// Adds a child to this node in the AST
    pub fn add_child(&mut self, child: NodeRef<T>) {
        self.children.push(child);
    }

    /// Gets the children of this node in the AST, if any
    pub fn get_children(&self) -> &Vec<NodeRef<T>> {
        &self.children
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    /// Compares two `Node` instances for equality.
    fn eq(&self, other: &Self) -> bool {
        // Compare `parent`, `children`, and `data` fields for equality.
        self.parent == other.parent && self.children == other.children && self.data == other.data
    }
}

/// A struct representing an Abstract Syntax Tree (AST)
#[derive(Debug)]
pub struct AST<T> {
    /// The root node of the AST
    root: NodeRef<T>,
}

impl<T> AST<T> {
    /// Creates a new `AST` with an empty root node
    pub fn new() -> AST<T> {
        let root = Node::new();
        AST { root }
    }

    /// Sets the root node of the AST
    pub fn set_root(&mut self, root: NodeRef<T>) {
        self.root = root;
    }

    /// Gets the root node of the AST
    pub fn get_root(&self) -> &NodeRef<T> {
        &self.root
    }

    /// Performs a depth-first traversal of the AST, calling the given closure on each node
    pub fn traverse<F>(&self, mut f: F)
    where
        F: FnMut(&NodeRef<T>),
    {
        let mut stack = vec![self.root.clone()];
        while let Some(node) = stack.pop() {
            f(&node);
            stack.extend(node.borrow().get_children().iter().cloned().rev());
        }
    }
}
