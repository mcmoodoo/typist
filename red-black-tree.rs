use std::cmp::Ordering;
use std::fmt::{self, Debug};

#[derive(Debug, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
struct Node<K: Ord, V> {
    key: K,
    value: V,
    color: Color,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> Node<K, V> {
    fn new(key: K, value: V, color: Color) -> Self {
        Node {
            key,
            value,
            color,
            left: None,
            right: None,
        }
    }
}

pub struct RedBlackTree<K: Ord, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: Ord + Debug, V: Debug> RedBlackTree<K, V> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.root = Some(self.insert_node(self.root.take(), key, value));
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
    }

    fn insert_node(&self, node: Option<Box<Node<K, V>>>, key: K, value: V) -> Box<Node<K, V>> {
        if let Some(mut current) = node {
            match key.cmp(&current.key) {
                Ordering::Less => {
                    current.left = Some(self.insert_node(current.left.take(), key, value));
                }
                Ordering::Greater => {
                    current.right = Some(self.insert_node(current.right.take(), key, value));
                }
                Ordering::Equal => {
                    current.value = value;
                }
            }
            self.balance(current)
        } else {
            Box::new(Node::new(key, value, Color::Red))
        }
    }

    fn balance(&self, mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        // Rotate left if right child is red and left child is not red
        if self.is_red(&node.right) && !self.is_red(&node.left) {
            node = self.rotate_left(node);
        }

        // Rotate right if left child and its left child are red
        if self.is_red(&node.left) && self.is_red(&node.left.as_ref().unwrap().left) {
            node = self.rotate_right(node);
        }

        // Flip colors if both children are red
        if self.is_red(&node.left) && self.is_red(&node.right) {
            self.flip_colors(&mut node);
        }

        node
    }

    fn is_red(&self, node: &Option<Box<Node<K, V>>>) -> bool {
        match node {
            Some(n) => n.color == Color::Red,
            None => false,
        }
    }

    fn rotate_left(&self, mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut new_root = node.right.take().unwrap();
        node.right = new_root.left.take();
        new_root.left = Some(node);
        new_root.color = new_root.left.as_ref().unwrap().color;
        new_root.left.as_mut().unwrap().color = Color::Red;
        new_root
    }

    fn rotate_right(&self, mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut new_root = node.left.take().unwrap();
        node.left = new_root.right.take();
        new_root.right = Some(node);
        new_root.color = new_root.right.as_ref().unwrap().color;
        new_root.right.as_mut().unwrap().color = Color::Red;
        new_root
    }

    fn flip_colors(&self, node: &mut Box<Node<K, V>>) {
        node.color = match node.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };

        if let Some(ref mut left) = node.left {
            left.color = match left.color {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };
        }

        if let Some(ref mut right) = node.right {
            right.color = match right.color {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };
        }
    }
}

fn main() {
    let mut tree = RedBlackTree::new();
    tree.insert(10, "ten");
    tree.insert(20, "twenty");
    tree.insert(15, "fifteen");

    println!("Tree after insertions: {:?}", tree);
}
