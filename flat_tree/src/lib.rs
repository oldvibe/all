use std::collections::BTreeSet;
pub fn flatten_tree<T: ToOwned<Owned = T> + std::fmt::Debug + Clone >(tree: &BTreeSet<T>) -> Vec<T> {
    let mut res: Vec<T> = Vec::new();
    for i in tree {
        println!("{:?}", i);
        res.push(i.clone());
    }
    res
}
