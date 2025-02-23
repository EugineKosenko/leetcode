use std::env;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self { Self { val, left: None, right: None } }
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
fn find(
    preorder: &mut Peekable<IntoIter<i32>>,
    postorder: &mut Peekable<IntoIter<i32>>)
    -> Option<Rc<RefCell<TreeNode>>> {
    let Some(prevalue) = preorder.next() else { return None; };
    let mut result = TreeNode::new(prevalue);
    if prevalue == *postorder.peek().unwrap() {
        postorder.next().unwrap();
        return Some(Rc::new(RefCell::new(result)));
    }
    result.left = find(preorder, postorder);
    if prevalue != *postorder.peek().unwrap() { result.right = find(preorder, postorder); }
    let postvalue = postorder.next().unwrap();
    debug_assert_eq!(prevalue, postvalue);
    Some(Rc::new(RefCell::new(result)))
}

fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    find(&mut preorder.into_iter().peekable(), &mut postorder.into_iter().peekable())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let preorder = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let postorder = args[2]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{:?}", TreeNode::show(&construct_from_pre_post(preorder, postorder)));
}
