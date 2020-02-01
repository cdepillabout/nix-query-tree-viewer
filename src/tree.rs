#[derive(Clone, Debug, PartialEq)]
pub struct Tree<T> {
    pub item: T,
    pub children: Vec<Tree<T>>,
}

impl<T> Tree<T> {
    pub fn new(item: T, children: Vec<Tree<T>>) -> Tree<T> {
        Tree { item, children }
    }
    pub fn singleton(item: T) -> Tree<T> {
        Tree::new(item, vec![])
    }
}

impl<T> Tree<T>
where
    T: Clone,
{
    pub fn append(&mut self, new_child_tree: Tree<T>) {
        self.children.push(new_child_tree);
    }
}
