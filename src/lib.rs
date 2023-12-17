
#[derive(Debug)]
pub struct ApterTree<T> {
    values: Vec<T>,
    edges: Vec<usize>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ApterTreeNodeRef(usize);

impl ApterTreeNodeRef {

    fn idx(&self) -> usize {
        self.0
    }

    fn is_root(&self) -> bool {
        self.0 == 0
    }

}

impl<T> ApterTree<T> {

    pub fn new(root: T) -> Self {
        ApterTree {
            values: vec![root],
            edges: vec![0]
        }
    }

    pub fn with_capacity(root: T, capacity: usize) -> Self {
        let mut values = Vec::with_capacity(capacity);
        let mut edges  = Vec::with_capacity(capacity);
        values.push(root);
        edges.push(0);
        ApterTree { values, edges }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn root(&self) -> ApterTreeNodeRef {
        ApterTreeNodeRef(0)
    }

    pub fn val(&self, node: ApterTreeNodeRef) -> &T {
        &self.values[node.idx()]
    }

    pub fn val_mut(&mut self, node: ApterTreeNodeRef) -> &mut T {
        &mut self.values[node.idx()]
    }
    
    pub fn parent(&self, child: ApterTreeNodeRef) -> Option<ApterTreeNodeRef> {
        if child.is_root() {
            None }
        else {
            Some(ApterTreeNodeRef(self.edges[child.idx()]))
        }
    }

    pub fn push_child(&mut self, parent: ApterTreeNodeRef, val: T) -> ApterTreeNodeRef {
        self.values.push(val);
        self.edges.push(parent.idx());
        ApterTreeNodeRef(self.values.len()-1)
    }

    pub fn scan_children(&self, parent: ApterTreeNodeRef) -> Children<T> {
        Children::new(self, parent)
    }

}

pub struct Children<'t, T> {
    tree: &'t ApterTree<T>,
    parent: ApterTreeNodeRef,
    idx: usize
}

impl<'t, T> Children<'t, T> {

    fn new(tree: &'t ApterTree<T>, parent: ApterTreeNodeRef) -> Self {
        let idx = parent.idx() + 1;
        Children {
            tree, parent, idx
        }
    }

}

impl<'t, T> Iterator for Children<'t, T> {

    type Item = ApterTreeNodeRef;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.idx..self.tree.len() {
            if self.tree.edges[i] == self.parent.idx() {
                self.idx = i + 1;
                return Some(ApterTreeNodeRef(i))
            }
        }
        None
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut tree = ApterTree::<&'static str>::new("ROOT");
        let a = tree.push_child(tree.root(), "A");
        tree.push_child(tree.root(), "B");
        tree.push_child(tree.root(), "C");
        let a1 = tree.push_child(a, "A1");
        let a2 = tree.push_child(a, "A2");
        let a3 = tree.push_child(a, "A3");
        println!("{:?}", tree);
        assert_eq!(tree.scan_children(a).collect::<Vec<_>>(), vec![a1, a2, a3]);
    }

}
