fn max_depth(root: Option<&Box<i32>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left());
            let right_depth = max_depth(node.right());
            left_depth.max(right_depth) + 1
        }
        None => 0,
    }
}

trait NodeExt {
    fn left(&self) -> Option<&Box<i32>>;
    fn right(&self) -> Option<&Box<i32>>;
}

impl NodeExt for Box<i32> {
    fn left(&self) -> Option<&Box<i32>> {
        None
    }

    fn right(&self) -> Option<&Box<i32>> {
        None
    }
}

fn main() {
    // Example usage:
    let root = Some(Box::new(3));
    println!("Maximum depth of the tree: {}", max_depth(root.as_ref()));
}
