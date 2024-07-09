use std::cell::RefCell;
use std::rc::Rc;

pub type NodeRef<T> = Rc<RefCell<Node<T>>>;

/// A struct representing a node in an Abstract Syntax Tree (AST)
#[derive(Debug)]
pub struct Node<T> {
    /// The ID of this node in the AST
    id: Option<String>,

    /// The parent of this node in the AST, if any
    parent: Option<NodeRef<T>>,

    /// The children of this node in the AST, if any
    children: Vec<NodeRef<T>>,

    /// The data associated with this node in the AST, if any
    data: Option<T>,
}

impl<T: Clone> Node<T> {
    /// Creates a new `Node` with specified initial ID, no parent, no children, no data
    pub fn new(id: Option<String>) -> NodeRef<T> {
        Rc::new(RefCell::new(Node {
            id,
            parent: None,
            children: Vec::new(),
            data: None,
        }))
    }

    /// Sets the node ID
    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
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
        // Add the child to the children list
        self.children.push(child);
    }

    /// Gets the children of this node in the AST, if any
    pub fn get_children(&self) -> &Vec<NodeRef<T>> {
        &self.children
    }

    /// Gets the ID of this node in the AST
    pub fn get_id(&self) -> &Option<String> {
        &self.id
    }

    /// Recursively gets data from the children of this node in the AST
    pub fn get_data_from_children(&self) -> Vec<T> {
        let mut data = Vec::new();
        for child in &self.children {
            if let Some(child_data) = child.borrow().get_data().cloned() {
                data.push(child_data);
            }
            data.extend(child.borrow().get_data_from_children());
        }
        data
    }
}

/// Implement the Clone trait for Node struct
impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node {
            id: self.id.clone(),
            parent: self.parent.clone(),
            children: self.children.iter().cloned().collect(),
            data: self.data.clone(),
        }
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
#[derive(Debug, Clone)]
pub struct AST<T> {
    /// The root node of the AST
    root: NodeRef<T>,
}

impl<T: Clone> AST<T> {
    /// Creates a new `AST` with an empty root node identified by the ID "root"
    pub fn new() -> AST<T> {
        let root = Node::new(Some("root".to_string()));
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

    /// Searches for a node with the given ID in the AST and returns it if found
    ///
    /// This function performs a depth-first search (DFS) starting from the root node,
    /// looking for a node with a matching ID. If such a node is found, it returns
    /// a cloned instance of that node. Otherwise, it returns `None`
    pub fn get_node_by_id(&self, id: &str) -> Option<Node<T>> {
        // Initialize a stack for DFS with the root node
        let mut stack = vec![self.root.clone()];

        // Continue processing until the stack is empty
        while let Some(node_ref) = stack.pop() {
            // Borrow the node inside the RefCell to access its fields
            let node = node_ref.borrow();

            // Check if the current node's ID matches the target ID
            if let Some(node_id) = &node.get_id() {
                if node_id == id {
                    // If a match is found, return a clone of the node
                    return Some(node.clone());
                }
            }

            // If no match, extend the stack with the children of the current node
            // Cloning the child references to maintain stack ownership
            stack.extend(node.children.iter().cloned().rev());
        }

        // If no matching node is found, return None
        None
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
