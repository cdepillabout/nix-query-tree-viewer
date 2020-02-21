use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

#[derive(Clone, Debug, Eq, PartialEq)]
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

    /// Lookup the item in the `Tree` that corresponds to the given `Path`.
    pub fn lookup(&self, path: Path) -> Option<&T> {
        match path.split_front() {
            None => Some(&self.item),
            Some((index, child_path)) => match self.children.get(index) {
                None => None,
                Some(child_tree) => child_tree.lookup(child_path),
            },
        }
    }

    /// Similar to `path_map`, but take a function for mapping an item in the tree to an
    /// alternative type to use to construct the `TreePathMap`.
    ///
    /// This is useful when there is some information in the item for the `Tree` that
    /// can be thrown away when constructing the `TreePathMap`.
    pub fn path_map_map<U>(&self, f: &dyn Fn(&T) -> U) -> TreePathMap<U>
    where
        U: Eq + Hash,
    {
        let mut map = TreePathMap::new();
        let root_path = Path::new();
        map.insert(f(&self.item), &root_path);
        map.insert_children_map(&self.children, &root_path, f);
        map
    }
}

impl<T> Tree<T>
where
    T: Clone + Eq + Hash,
{
    /// Create a `TreePathMap` for the elements in the `Tree`.
    pub fn path_map(&self) -> TreePathMap<T> {
        self.path_map_map(&|i| i.clone())
    }
}

/// This represents the path through a `Tree<T>` to a given node.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Path(pub VecDeque<usize>);

impl Path {
    pub fn split_front(mut self) -> Option<(usize, Path)> {
        let option_front_elem: Option<usize> = self.0.pop_front();
        match option_front_elem {
            None => None,
            Some(i) => Some((i, self)),
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

impl<T> From<T> for Path
where
    T: Into<VecDeque<usize>>,
{
    fn from(other: T) -> Path {
        Path(other.into())
    }
}

/// This is a mapping of items in `Tree` to their `Path`s.  A single item in the `Tree` can have
/// multiple `Path`s to it if it is in the `Tree` multiple times.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TreePathMap<U>(HashMap<U, Vec<Path>>)
where
    U: Eq + Hash;

impl<U> TreePathMap<U>
where
    U: Eq + Hash,
{
    pub fn new() -> TreePathMap<U> {
        TreePathMap(HashMap::new())
    }

    /// Insert a mapping from `U` to `Path`.
    pub fn insert(&mut self, k: U, path: &Path) {
        self.0
            .entry(k)
            .and_modify(|paths| paths.push(path.clone()))
            .or_insert_with(|| vec![path.clone()]);
    }

    /// Lookup the first `Path` for a given item.
    pub fn lookup_first(&self, k: &U) -> Option<&Path> {
        let option_paths: Option<&Vec<Path>> = self.0.get(k);
        option_paths.and_then(|vec: &Vec<Path>| vec.first())
    }
}

impl<U> TreePathMap<U>
where
    U: Eq + Hash,
{
    /// Insert child `Tree`s starting at `Path`.
    ///
    /// The function `f` will map the items in the `T` to an alternative type.
    fn insert_children_map<T>(
        &mut self,
        children: &[Tree<T>],
        path: &Path,
        f: &dyn Fn(&T) -> U,
    ) {
        for (i, child) in children.iter().enumerate() {
            let child_path = path.push_back(i);
            self.insert(f(&child.item), &child_path);
            self.insert_children_map(&child.children, &child_path, f);
        }
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
        let tree = Tree::new(
            "root",
            vec![
                Tree::singleton("0"),
                Tree::singleton("1"),
                Tree::new(
                    "2",
                    vec![Tree::singleton("2-0"), Tree::singleton("2-1")],
                ),
            ],
        );

        let path1 = vec![3].into();
        let path2 = vec![0, 1].into();
        let path3 = vec![1, 0].into();
        let path4 = vec![2, 2].into();
        let path5 = vec![2, 0, 3].into();
        let path6 = vec![0, 1, 2, 3, 4].into();

        assert_eq!(tree.lookup(&path1), None);
        assert_eq!(tree.lookup(&path2), None);
        assert_eq!(tree.lookup(&path3), None);
        assert_eq!(tree.lookup(&path4), None);
        assert_eq!(tree.lookup(&path5), None);
        assert_eq!(tree.lookup(&path6), None);
    }

    #[test]
    fn test_lookup_find_item() {
        let tree: Tree<String> = Tree::new(
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
        let path2_0 = vec![2, 0].into();
        let path2_1 = vec![2, 1].into();
        let path2_1_0 = vec![2, 1, 0].into();
        let path2_1_1 = vec![2, 1, 1].into();

        assert_eq!(tree.lookup(&path_root).map(String::deref), Some("root"));
        assert_eq!(tree.lookup(&path0).map(String::deref), Some("0"));
        assert_eq!(tree.lookup(&path1).map(String::deref), Some("1"));
        assert_eq!(tree.lookup(&path2).map(String::deref), Some("2"));
        assert_eq!(tree.lookup(&path2_0).map(String::deref), Some("2-0"));
        assert_eq!(tree.lookup(&path2_1).map(String::deref), Some("2-1"));
        assert_eq!(tree.lookup(&path2_1_0).map(String::deref), Some("2-1-0"));
        assert_eq!(tree.lookup(&path2_1_1).map(String::deref), Some("2-1-1"));
    }

    #[test]
    fn test_tree_path_map_from_tree_all_unique() {
        let tree: Tree<String> = Tree::new(
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

        let res_tree_path_map: TreePathMap<String> = tree.path_map();

        let mut actual_tree_path_map: HashMap<String, Vec<Path>> =
            HashMap::new();

        actual_tree_path_map.insert("root".into(), vec![Path::new()]);
        actual_tree_path_map.insert("0".into(), vec![vec![0].into()]);
        actual_tree_path_map.insert("1".into(), vec![vec![1].into()]);
        actual_tree_path_map.insert("2".into(), vec![vec![2].into()]);
        actual_tree_path_map.insert("2-0".into(), vec![vec![2, 0].into()]);
        actual_tree_path_map.insert("2-1".into(), vec![vec![2, 1].into()]);
        actual_tree_path_map.insert("2-1-0".into(), vec![vec![2, 1, 0].into()]);
        actual_tree_path_map.insert("2-1-1".into(), vec![vec![2, 1, 1].into()]);

        assert_eq!(res_tree_path_map, TreePathMap(actual_tree_path_map));
    }

    #[test]
    fn test_tree_path_map_from_tree() {
        let tree: Tree<String> = Tree::new(
            "cat".into(), // root
            vec![
                Tree::singleton("dog".into()), // 0
                Tree::singleton("cat".into()), // 1
                Tree::new(
                    "mouse".into(), // 2
                    vec![
                        Tree::singleton("fish".into()), // 2-0
                        Tree::new(
                            "fish".into(), // 2-1
                            vec![
                                Tree::singleton("dog".into()), // 2-1-0
                                Tree::singleton("cat".into()), // 2-1-1
                            ],
                        ),
                    ],
                ),
            ],
        );

        let res_tree_path_map: TreePathMap<String> = tree.path_map();

        let mut actual_tree_path_map: HashMap<String, Vec<Path>> =
            HashMap::new();

        actual_tree_path_map.insert(
            "cat".into(),
            vec![Path::new(), vec![1].into(), vec![2, 1, 1].into()],
        );
        actual_tree_path_map
            .insert("dog".into(), vec![vec![0].into(), vec![2, 1, 0].into()]);
        actual_tree_path_map.insert("mouse".into(), vec![vec![2].into()]);
        actual_tree_path_map
            .insert("fish".into(), vec![vec![2, 0].into(), vec![2, 1].into()]);

        assert_eq!(res_tree_path_map, TreePathMap(actual_tree_path_map));
    }
}
