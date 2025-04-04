use std::env;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;
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
    fn from_vec(vs: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::from_vec_(0, &vs)
    }
    fn from_vec_(i: usize, vs: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        vs.get(i).and_then(|v| v.map(|v| Rc::new(RefCell::new(TreeNode {
            val: v,
            left: Self::from_vec_(2*i+1, vs),
            right: Self::from_vec_(2*i+2, vs)
        }))))
    }
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
    fn _print(root: &Option<Rc<RefCell<TreeNode>>>) {
        Self::print_(root, 0);
    }
    
    fn print_(root: &Option<Rc<RefCell<TreeNode>>>, indent: usize) {
        match root {
            None => println!("{}None", " ".repeat(indent)),
            Some(root) => {
                let root = root.borrow();
                println!("{}Some({})", " ".repeat(indent), root.val);
                Self::print_(&root.left, indent + 1);
                Self::print_(&root.right, indent + 1);
            }
        }
    }
}
fn find(depth: usize, root: Option<Rc<RefCell<TreeNode>>>) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
    match root {
        None => (depth, root),
        Some(ref root_) => {
            let root_ = root_.borrow();
            let left = find(depth+1, root_.left.clone());
            let right = find(depth+1, root_.right.clone());
            match left.0.cmp(&right.0) {
                Ordering::Greater => left,
                Ordering::Less => right,
                Ordering::Equal => (left.0, root.clone())
            }
        }
    }
}

fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    find(0, root).1.clone()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let root = TreeNode::from_vec(
        &args[1]
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|item| item.parse().ok())
            .collect());
    println!("{:?}", TreeNode::show(&lca_deepest_leaves(root)));
}
