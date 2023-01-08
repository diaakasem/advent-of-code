#[derive(Debug)]
pub struct Tree {
    height: u32,
    column: u32,
    row: u32,
}

impl Tree {
    pub fn new(height: u32, column: u32, row: u32) -> Tree {
        Tree {
            height,
            column,
            row,
        }
    }
}

pub struct TreeGrid {
    max_rows: u32,
    max_columns: u32,
    trees: Vec<Tree>,
}

impl TreeGrid {

    pub fn new() -> Self {
        Self {
            max_rows: 0,
            max_columns: 0,
            trees: Vec::new(),
        }
    }

    pub fn add_tree(&mut self, tree: Tree) {
        if tree.row > self.max_rows {
            self.max_rows = tree.row;
        }
        if tree.column > self.max_columns {
            self.max_columns = tree.column;
        }
        // println!("Adding tree: {:?}", tree);
        self.trees.push(tree);
    }

    fn get_trees(&self) -> &Vec<Tree> {
        &self.trees
    }

    fn get_tree(&self, column: u32, row: u32) -> Option<&Tree> {
        self.trees.iter().find(|tree| tree.column == column && tree.row == row)
    }

    fn get_trees_up(&self, column: u32, row: u32) -> Vec<&Tree> {
        self.trees.iter().filter(|tree| tree.column == column && tree.row < row).rev().collect()
    }

    fn get_trees_down(&self, column: u32, row: u32) -> Vec<&Tree> {
        self.trees.iter().filter(|tree| tree.column == column && tree.row > row).collect()
    }

    fn get_trees_left(&self, column: u32, row: u32) -> Vec<&Tree> {
        self.trees.iter().filter(|tree| tree.row == row && tree.column < column).rev().collect()
    }

    fn get_trees_right(&self, column: u32, row: u32) -> Vec<&Tree> {
        self.trees.iter().filter(|tree| tree.row == row && tree.column > column).collect()
    }

    fn is_tree_visible(&self, column: u32, row: u32) -> bool {
        if row == 0 || column == 0 || row == self.max_rows || column == self.max_columns {
            return true;
        }
        let trees_up = self.get_trees_up(column, row);
        let trees_down = self.get_trees_down(column, row);
        let trees_left = self.get_trees_left(column, row);
        let trees_right = self.get_trees_right(column, row);
        let this_tree = self.get_tree(column, row).unwrap();
        let is_visible_up = trees_up.iter().all(|tree| tree.height < this_tree.height);
        let is_visible_down = trees_down.iter().all(|tree| tree.height < this_tree.height);
        let is_visible_left = trees_left.iter().all(|tree| tree.height < this_tree.height);
        let is_visible_right = trees_right.iter().all(|tree| tree.height < this_tree.height);
        is_visible_left || is_visible_right || is_visible_up || is_visible_down
    }

    fn get_viewable_trees_count(&self, tree: &Tree, view: Vec<&Tree>) -> u32 {
        let mut viewable_trees_count = 0;
        for t in view {
            if t.height < tree.height {
                viewable_trees_count += 1;
            }
            if t.height >= tree.height {
                viewable_trees_count += 1;
                break;
            }
        }
        viewable_trees_count
    }

    fn get_scenic_view(&self, column: u32, row: u32) -> u32 {
        let this_tree = self.get_tree(column, row).unwrap();
        let trees_up = self.get_trees_up(column, row);
        let trees_down = self.get_trees_down(column, row);
        let trees_left = self.get_trees_left(column, row);
        let trees_right = self.get_trees_right(column, row);

        let left_view = self.get_viewable_trees_count(&this_tree, trees_left);
        let right_view = self.get_viewable_trees_count(&this_tree, trees_right);
        let up_view = self.get_viewable_trees_count(&this_tree, trees_up);
        let down_view = self.get_viewable_trees_count(&this_tree, trees_down);
        let scenic_view = left_view * right_view * up_view * down_view;

        scenic_view
    }

    pub fn get_visible_trees_count(&self) -> u32 {
        let mut count = 0;
        // println!("max_rows: {}, max_columns: {}", self.max_rows, self.max_columns);
        for row in 0..=self.max_rows {
            for column in 0..=self.max_columns {
                if self.is_tree_visible(column, row) {
                    // println!("Tree at ({}, {}) is visible", column, row);
                    count += 1;
                }
            }
        }
        count
    }

    pub fn get_max_scenic_view(&self) -> u32 {
        let mut max_scenic_view = 0;
        for row in 0..=self.max_rows {
            for column in 0..=self.max_columns {
                let scenic_view = self.get_scenic_view(column, row);
                if scenic_view > max_scenic_view {
                    max_scenic_view = scenic_view;
                }
            }
        }
        max_scenic_view
    }

}
