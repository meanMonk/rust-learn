// Struct with dynamic data

use crate::greet;

// NOTE: A most powerful technique is a struct that contain references to itself.

// So in rust we can create type like.

// why do we need nodeBox:
// Rust does not do "NULL" so it's clearly a job for "Option".
// but you can't put a 'Node' in that option as we don't know the size of Node.
// So it's job for "Box" since it contains an allocated pointer to data and always has fixed size.

type NodeBox = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    payload: String,
    left: NodeBox,
    right: NodeBox,
}

impl Node {
    // creating new Node
    fn new(s: &str) -> Node {
        Node {
            payload: s.to_string(),
            left: None,
            right: None,
        }
    }

    // creating box of Node or data.
    fn boxer(node: Node) -> NodeBox {
        Some(Box::new(node))
    }

    // setting left node
    fn set_left(&mut self, node: Node) {
        self.left = Self::boxer(node);
    }
    // setting right node
    fn set_right(&mut self, node: Node) {
        self.right = Self::boxer(node);
    }

    // we must now work out a use for this tree
    // note string can be ordered 'alphabetical order'
    // so setting node by sorting alphabetical
    // small goes to left and other to right node.

    fn insert(&mut self, data: &str) {
        if data < &self.payload {
            match self.left {
                // if left has box go dipper
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data)),
            }
        } else {
            match self.right {
                // if righ thas box go dipper else set node to right branch None
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data)),
            }
        }
    }

    // time for visit - in order traversal
    // we visit on left and do some ops and then visit to the right.

    fn visit(&self) {
        //`ref - if let`  use same rules as match
        if let Some(ref left) = self.left {
            left.visit();
        }
        println!("'{}'", self.payload);
        if let Some(ref right) = self.right {
            right.visit();
        }
    }
}

fn binary_tree_prepare() {
    let mut root = Node::new("root");
    root.set_left(Node::new("left-l1"));
    root.set_right(Node::new("right-l1"));

    // '#' means extended
    println!("{:#?}", root);
}

fn binary_tree_one() {
    let mut count_tree = Node::new("apple");
    count_tree.insert("four");
    count_tree.insert("one");
    count_tree.insert("two");
    count_tree.insert("five");
    count_tree.insert("seven");
    count_tree.insert("eight");

    println!("Count Tree {:#?}", count_tree);

    // visit the tree in order

    count_tree.visit();
}

// what will happened if `root` is dropped?
// - All fields are dropped similarly for branches of the tree,
// if the `branches` drops they drop their fields and so on.

pub fn main() {
    greet::greet("Struct Dynamic Data |  Binary Tree 🌴");

    binary_tree_prepare();
    binary_tree_one();
}
