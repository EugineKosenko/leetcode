use std::env;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;
use std::iter::Peekable;
use std::str::Chars;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn _new(val: i32) -> Self { Self { val, left: None, right: None } }
    fn show(root: &Option<Rc<RefCell<Self>>>) -> Vec<Option<i32>> {
        match root {
            None => vec![],
            Some(root) => {
                let root = root.borrow();
                let mut result = vec![Some(root.val)];
                Self::show_(root, &mut result);
                result
            }
        }
    }
    fn show_(root: Ref<'_, Self>, result: &mut Vec<Option<i32>>) {
        if root.left.is_none() && root.right.is_none() { return; }
        result.push(root.left.as_ref().map(|n| n.borrow().val));
        if let Some(right) = &root.right { result.push(Some(right.borrow().val)); }
        match &root.left {
            None => { result.push(None); }
            Some(left) => { Self::show_(left.borrow(), result); }
        }
        if let Some(right) = &root.right { Self::show_(right.borrow(), result); }
    }
}
struct Traversal<'a>(Peekable<Chars<'a>>);

impl<'a> Traversal<'a> {
    fn new(s: &'a str) -> Self { Self(s.chars().peekable()) }
    
}

impl Iterator for Traversal<'_> {
    type Item = (usize, i32);
    fn next(&mut self) -> Option<Self::Item> {
        let Some(&c) = self.0.peek() else { return None; };
        let (mut depth, mut value) = (0, 0);
        if c == '-' {
            while let Some('-') = self.0.peek() { self.0.next().unwrap(); depth += 1; }
        }
        while let Some('0'..='9') = self.0.peek() {
            value = 10 * value + self.0.next().unwrap().to_digit(10).unwrap() as i32;
        }
        Some((depth, value))
    }
}
fn find(traversal: &mut Peekable<Traversal>, depth: usize) -> Option<Rc<RefCell<TreeNode>>> {
    let Some((ndepth, _)) = traversal.peek() else { return None; };
    match depth.cmp(&ndepth) {
        Ordering::Greater => None,
        Ordering::Equal => {
            let (_, val) = traversal.next().unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val, left: find(traversal, depth + 1), right: find(traversal, depth + 1)
            })))
        },
        Ordering::Less => panic!("Invalid traversal")
    }
}

fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
    find(&mut Traversal::new(&traversal).peekable(), 0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let traversal = args[1].to_string();
    println!("{:?}", TreeNode::show(&recover_from_preorder(traversal)));
}
