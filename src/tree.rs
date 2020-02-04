use std::collections::VecDeque;

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

    pub fn lookup(&self, path: Path) -> Option<&T> {
        match path.split_front() {
            None => Some(&self.item),
            Some((index, child_path)) => {
                match self.children.get(index) {
                    None => None,
                    Some(child_tree) => child_tree.lookup(child_path),
                }
            }
        }
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

#[derive(Clone, Debug, PartialEq)]
pub struct Path(pub VecDeque<usize>);

impl Path {
    pub fn split_front(&self) -> Option<(usize, Path)> {
        let mut new_path = self.clone();
        let option_front_elem: Option<usize> = new_path.0.pop_front();
        match option_front_elem {
            None => None,
            Some(i) => Some((i, new_path)),
        }
    }

    pub fn push_back(&self, value: usize) -> Path {
        let mut new_path = self.clone();
        new_path.0.push_back(value);
        new_path
    }

    pub fn new() -> Self {
        Path(VecDeque::new())
    }
}

impl From<Vec<usize>> for Path {
    fn from(other: Vec<usize>) -> Path {
        Path(other.into())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::ops::Deref;

    #[test]
    fn test_path_split_front_empty() {
        let res = Path::new().split_front();
        assert_eq!(res, None);
    }

    #[test]
    fn test_path_push_back() {
        let res = Path::new().push_back(1).push_back(2).push_back(3);

        let mut actual_vec = VecDeque::new();
        actual_vec.push_back(1);
        actual_vec.push_back(2);
        actual_vec.push_back(3);
        let actual_path = Path(actual_vec);

        assert_eq!(res, actual_path);
    }

    #[test]
    fn test_path_split_front_nonempty() {
        let path = Path::new().push_back(1).push_back(2).push_back(3);
        let res = path.split_front();

        let actual_path = Path::new().push_back(2).push_back(3);
        let actual = Some((1, actual_path));

        assert_eq!(res, actual);
    }

    #[test]
    fn test_lookup_no_item() {
        let tree =
            Tree::new(
                "root",
                vec![
                    Tree::singleton("0"),
                    Tree::singleton("1"),
                    Tree::new(
                        "2",
                        vec![
                            Tree::singleton("2-0"),
                            Tree::singleton("2-1"),
                        ],
                    ),
                ],
            );

        let path1 = vec![3].into();
        let path2 = vec![0,1].into();
        let path3 = vec![1,0].into();
        let path4 = vec![2,2].into();
        let path5 = vec![2,0,3].into();
        let path6 = vec![0,1,2,3,4].into();

        assert_eq!(tree.lookup(path1), None);
        assert_eq!(tree.lookup(path2), None);
        assert_eq!(tree.lookup(path3), None);
        assert_eq!(tree.lookup(path4), None);
        assert_eq!(tree.lookup(path5), None);
        assert_eq!(tree.lookup(path6), None);
    }

    #[test]
    fn test_lookup_find_item() {
        let tree: Tree<String> =
            Tree::new(
                "root".into(),
                vec![
                    Tree::singleton("0".into()),
                    Tree::singleton("1".into()),
                    Tree::new(
                        "2".into(),
                        vec![
                            Tree::singleton("2-0".into()),
                            Tree::new(
                                "2-1".into(),
                                vec![
                                    Tree::singleton("2-1-0".into()),
                                    Tree::singleton("2-1-1".into()),
                                ],
                            ),
                        ],
                    ),
                ],
            );

        let path_root = vec![].into();
        let path0 = vec![0].into();
        let path1 = vec![1].into();
        let path2 = vec![2].into();
        let path2_0 = vec![2,0].into();
        let path2_1 = vec![2,1].into();
        let path2_1_0 = vec![2,1,0].into();
        let path2_1_1 = vec![2,1,1].into();

        assert_eq!(tree.lookup(path_root).map(String::deref), Some("root"));
        assert_eq!(tree.lookup(path0).map(String::deref), Some("0"));
        assert_eq!(tree.lookup(path1).map(String::deref), Some("1"));
        assert_eq!(tree.lookup(path2).map(String::deref), Some("2"));
        assert_eq!(tree.lookup(path2_0).map(String::deref), Some("2-0"));
        assert_eq!(tree.lookup(path2_1).map(String::deref), Some("2-1"));
        assert_eq!(tree.lookup(path2_1_0).map(String::deref), Some("2-1-0"));
        assert_eq!(tree.lookup(path2_1_1).map(String::deref), Some("2-1-1"));
    }
}
