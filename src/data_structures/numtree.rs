pub struct NumTree {
    tree: Vec<Vec<usize>>,
    root: usize,
}

pub struct NumTreeWeighted {
    tree: Vec<Vec<(usize, usize)>>,
    root: usize,
}

pub struct NumTreeExt {
    pub depth: Vec<usize>,
    pub height: Vec<usize>,
}

// every node is a vector which links to a list of nodes
// at 0th position of the vector it contains entry to its parent
// other entries points to child nodes

impl NumTree {
    pub fn new(n: usize, root: usize) -> Self {
        let mut tree = vec![vec![usize::MAX]; n];
        // root node's parent points to root
        tree[root][0] = root;

        Self { tree, root }
    }

    pub fn from(n: usize, list: &[(usize, usize)]) -> Self {
        let mut tree = vec![vec![usize::MAX]; n];
        // the first entry of the list is considered as root
        let root = list[0].0;
        tree[root][0] = root;

        for (parent, child) in list {
            tree[*child][0] = *parent;
            tree[*parent].push(*child);
        }

        Self { tree, root }
    }

    pub fn link(&mut self, parent: usize, child: usize) {
        self.tree[child][0] = parent;
        self.tree[parent].push(child);
    }

    pub fn get_depth_and_height(&self) -> NumTreeExt {
        let depth = vec![0usize; self.tree.len()];
        let height = vec![0usize; self.tree.len()];
        let mut result = NumTreeExt { depth, height };
        result.dfs(&self.tree, self.root);
        result
    }
}

impl NumTreeWeighted {
    pub fn new(n: usize, root: usize) -> Self {
        let mut tree = vec![vec![(usize::MAX, 0usize)]; n];
        // root node's parent points to root
        tree[root][0].0 = root;

        Self { tree, root }
    }

    pub fn get_depth_and_height(&self) -> NumTreeExt {
        let depth = vec![0usize; self.tree.len()];
        let height = vec![0usize; self.tree.len()];
        let mut result = NumTreeExt { depth, height };
        result.dfs_weighted(&self.tree, self.root);
        result
    }
}

impl NumTreeExt {
    // adding a node to the tree doesn't affect depth of other nodes
    // but it may or may not affect height of other nodes of the tree
    fn dfs(&mut self, tree: &Vec<Vec<usize>>, node: usize) {
        let mut it = tree[node].iter();
        // skipping the parent node
        it.next();

        for child in it {
            self.depth[*child] = self.depth[node] + 1;
            self.dfs(tree, *child);
            self.height[node] = usize::max(self.height[node], self.height[*child] + 1);
        }
    }

    fn dfs_weighted(&mut self, tree: &Vec<Vec<(usize, usize)>>, node: usize) {
        let mut it = tree[node].iter();
        // skipping the parent node
        it.next();

        for (child_id, weight) in it {
            self.depth[*child_id] = self.depth[node] + weight;
            self.dfs_weighted(tree, *child_id);
            self.height[node] = usize::max(self.height[node], self.height[*child_id] + weight);
        }
    }
}
