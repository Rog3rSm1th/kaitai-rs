use std::cell::RefCell;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

/// A struct representing a node in an Abstract Syntax Tree (AST)
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
    fn new() -> NodeRef<T> {
        Rc::new(RefCell::new(Node {
            parent: None,
            children: Vec::new(),
            data: None,
        }))
    }

    /// Sets the data associated with this node in the AST
    fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }

    /// Gets the data associated with this node in the AST, if any
    fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// Sets the parent of this node in the AST
    fn set_parent(&mut self, parent: NodeRef<T>) {
        self.parent = Some(parent);
    }

    /// Gets the parent of this node in the AST, if any
    fn get_parent(&self) -> Option<&NodeRef<T>> {
        self.parent.as_ref()
    }

    /// Adds a child to this node in the AST
    fn add_child(&mut self, child: NodeRef<T>) {
        self.children.push(child);
    }

    /// Gets the children of this node in the AST, if any
    fn get_children(&self) -> &Vec<NodeRef<T>> {
        &self.children
    }
}

/// A struct representing an Abstract Syntax Tree (AST)
pub struct AST<T> {
    /// The root node of the AST
    root: NodeRef<T>,
}

impl<T> AST<T> {
    /// Creates a new `AST` with the given root node
    pub fn new(root: NodeRef<T>) -> AST<T> {
        AST { root }
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
