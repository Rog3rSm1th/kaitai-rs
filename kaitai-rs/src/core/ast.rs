use std::cell::RefCell;
use std::rc::Rc;

pub type NodeRef = Rc<RefCell<Node>>;

/// Enum representing the type of a node
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    String,
    Integer,
    Array,
}

/// A struct representing a node in an Abstract Syntax Tree (AST)
#[derive(Debug)]
pub struct Node {
    /// The ID of this node in the AST
    id: Option<String>,

    /// The parent of this node in the AST, if any
    parent: Option<NodeRef>,

    /// The children of this node in the AST, if any
    children: Vec<NodeRef>,

    /// The data associated with this node in the AST, if any
    data: Option<Vec<u8>>,

    /// The type of this node in the AST
    node_type: Option<NodeType>,
}

impl Node {
    /// Creates a new `Node` with specified initial ID, no type, no parent, no children, no data
    pub fn new(id: Option<String>) -> NodeRef {
        Rc::new(RefCell::new(Node {
            id,
            parent: None,
            children: Vec::new(),
            data: None,
            node_type: None,
        }))
    }

    /// Sets the node ID
    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }

    /// Sets the data associated with this node in the AST
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = Some(data);
    }

    /// Gets the data associated with this node in the AST, if any
    pub fn get_data(&self) -> Option<&Vec<u8>> {
        self.data.as_ref()
    }

    /// Sets the parent of this node in the AST
    pub fn set_parent(&mut self, parent: NodeRef) {
        self.parent = Some(parent);
    }

    /// Gets the parent of this node in the AST, if any
    pub fn get_parent(&self) -> Option<&NodeRef> {
        self.parent.as_ref()
    }

    /// Adds a child to this node in the AST
    pub fn add_child(&mut self, child: NodeRef) {
        // Add the child to the children list
        self.children.push(child);
    }

    /// Gets the children of this node in the AST, if any
    pub fn get_children(&self) -> &Vec<NodeRef> {
        &self.children
    }

    /// Gets the ID of this node in the AST
    pub fn get_id(&self) -> &Option<String> {
        &self.id
    }

    /// Sets the type of this node in the AST
    pub fn set_node_type(&mut self, node_type: NodeType) {
        self.node_type = Some(node_type);
    }

    /// Gets the type of this node in the AST
    pub fn get_node_type(&self) -> Option<&NodeType> {
        self.node_type.as_ref()
    }

    /// Recursively gets data from the children of this node in the AST
    pub fn get_data_from_children(&self) -> Vec<Vec<u8>> {
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
impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            id: self.id.clone(),
            parent: self.parent.clone(),
            children: self.children.iter().cloned().collect(),
            data: self.data.clone(),
            node_type: self.node_type.clone(),
        }
    }
}

impl PartialEq for Node {
    /// Compares two `Node` instances for equality.
    fn eq(&self, other: &Self) -> bool {
        // Compare `parent`, `children`, `data`, and `node_type` fields for equality.
        self.parent == other.parent
            && self.children == other.children
            && self.data == other.data
            && self.node_type == other.node_type
    }
}

/// A struct representing an Abstract Syntax Tree (AST)
#[derive(Debug, Clone)]
pub struct AST {
    /// The root node of the AST
    root: NodeRef,
}

impl AST {
    /// Creates a new `AST` with an empty root node identified by the ID "root"
    pub fn new() -> AST {
        let root = Node::new(Some("root".to_string()));
        AST { root }
    }

    /// Sets the root node of the AST
    pub fn set_root(&mut self, root: NodeRef) {
        self.root = root;
    }

    /// Gets the root node of the AST
    pub fn get_root(&self) -> &NodeRef {
        &self.root
    }

    /// Searches for a node with the given ID in the AST and returns it if found
    ///
    /// This function performs a depth-first search (DFS) starting from the root node,
    /// looking for a node with a matching ID. If such a node is found, it returns
    /// a cloned instance of that node. Otherwise, it returns `None`
    pub fn get_node_by_id(&self, id: &str) -> Option<Node> {
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
        F: FnMut(&NodeRef),
    {
        let mut stack = vec![self.root.clone()];
        while let Some(node) = stack.pop() {
            f(&node);
            stack.extend(node.borrow().get_children().iter().cloned().rev());
        }
    }

    /// Prints the AST in a readable format
    pub fn print_ast(&self) {
        self.print_node(self.get_root(), 0, 0);
    }

    /// Helper function to print a node and its children recursively
    fn print_node(&self, node: &NodeRef, level: usize, index: usize) {
        let node_borrowed = node.borrow();
        let id_or_index = node_borrowed
            .get_id()
            .clone()
            .unwrap_or_else(|| format!("{}", index));
        if node_borrowed.get_children().is_empty() {
            // Print the node name or index with the appropriate indentation and its data
            let data = match (&node_borrowed.data, node_borrowed.get_node_type()) {
                // String
                (Some(d), Some(NodeType::String)) => {
                    format!("{:?}", String::from_utf8_lossy(d).trim())
                }
                // integer
                (Some(d), Some(NodeType::Integer)) => {
                    let integer = match d.len() {
                        1 => u8::from_le_bytes(d[..1].try_into().unwrap()) as u64,
                        2 => u16::from_le_bytes(d[..2].try_into().unwrap()) as u64,
                        4 => u32::from_le_bytes(d[..4].try_into().unwrap()) as u64,
                        8 => u64::from_le_bytes(d[..8].try_into().unwrap()) as u64,
                        _ => {
                            eprintln!("Invalid integer data: {:?}", d);
                            return;
                        }
                    };
                    // Array
                    format!("0x{:x}", integer)
                }
                (Some(d), _) => format!("{:?}", d),
                (None, _) => "None".to_string(),
            };
            println!("{}{}: {}", " ".repeat(level * 4), id_or_index, data);
        } else {
            println!("{}{}", " ".repeat(level * 4), id_or_index);
            // Continue traversing the children
            for (i, child) in node_borrowed.get_children().iter().enumerate() {
                self.print_node(child, level + 1, i);
            }
        }
    }
}
