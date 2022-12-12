#[derive(Debug)]
pub struct SegmentTree {
    nodes: Vec<Node>,
}

#[derive(Copy, Clone, Debug)]
pub struct Data {
    val: i32,
}

#[derive(Clone, Debug)]
struct Range {
    // Perhaps ranges should be usize and not i32
    // but that way we can't mark invalid ranges as -1
    start: i32,
    end: i32,
}

#[derive(Clone, Debug)]
struct Node {
    data: Data,           // Data contained in the node
    range: Range,         // The range this node refers to
    pending: Option<i32>, // Optional pending update on this node
}

impl SegmentTree {
    /**
     * Builds a segment tree on the provided data and returns it.
     */
    pub fn new(data: &Vec<i32>) -> Self {
        // Add padding in case the data vector's length is not a power of 2
        let smallest_exp = (data.len() as f64).log2().ceil() as u32;
        let tree_size = 2 * usize::pow(2, smallest_exp) - 1;

        let mut nodes: Vec<Node> = vec![
            Node {
                range: Range { start: -1, end: -1 },
                data: Data { val: 0 },
                pending: None
            };
            tree_size
        ];

        SegmentTree::build_tree(&mut nodes, 0, data, 0, (data.len() - 1) as i32);

        Self { nodes }
    }

    /**
     * Updates the elements in the range by val.
     */
    pub fn range_update(&mut self, i: usize, val: i32, start: i32, end: i32) {
        if i >= self.nodes.len() {
            panic!("Index out of bounds.");
        }

        if start > self.nodes[i].range.end || end < self.nodes[i].range.start {
            //No overlap
            return;
        }

        self.propagate_pending_update(i);

        if self.nodes[i].range.start >= start && self.nodes[i].range.end <= end {
            // Total overlap
            if self.nodes[i].range.start == self.nodes[i].range.end {
                // Leaf: update it
                self.nodes[i].data.val += val;
            } else {
                // Regular node: add update as pending
                self.nodes[i].pending = Some(val);
            }
            return;
        }

        // Partial overlap
        self.range_update(2 * i + 1, val, start, end);
        self.range_update(2 * i + 2, val, start, end);
    }

    /**
     * Pushes all of the tree's leaves in the out vector,
     *  propagating any pending update.
     */
    pub fn get_leaves(&mut self, i: usize, out: &mut Vec<i32>) {
        if i > self.nodes.len() {
            return;
        }

        self.propagate_pending_update(i);

        if self.nodes[i].range.start == self.nodes[i].range.end {
            out.push(self.nodes[i].data.val);
        } else {
            self.get_leaves(2 * i + 1, out);
            self.get_leaves(2 * i + 2, out);
        }
    }

    /**
     * Recursively build the segment tree
     */
    fn build_tree(nodes: &mut Vec<Node>, i: usize, data: &Vec<i32>, start: i32, end: i32) -> Data {
        if i >= nodes.len() || start == -1 || end == -1 {
            panic!("TODO");
        }

        // Leaf
        if start == end {
            nodes[i] = Node {
                data: Data {
                    // Leaves contain the array's elements
                    val: data[start as usize],
                },
                range: Range { start, end },
                pending: None,
            };

            return nodes[i].data;
        }

        //Non-leaf node
        let m: i32 = ((start as f64 + end as f64) / 2.0).floor() as i32;
        SegmentTree::build_tree(nodes, 2 * i + 1, data, start, m);
        SegmentTree::build_tree(nodes, 2 * i + 2, data, m + 1, end);

        // Create and insert the node
        nodes[i] = Node {
            // Non-leaves contain dummy data
            data: Data { val: 0 },
            range: Range { start, end },
            pending: None,
        };

        nodes[i].data
    }

    /**
     * Applies any pending update on node i and propagates
     * it to the node's children.
     */
    fn propagate_pending_update(&mut self, i: usize) {
        if i >= self.nodes.len() {
            // Out of bounds
            return;
        }

        let mut node = &mut self.nodes[i];

        match node.pending {
            Some(val) => {
                node.pending = None;

                // Update node if it's a leaf
                if node.range.start == node.range.end {
                    node.data.val += val;
                } else {
                    //Set pending update on children
                    let left_i = 2 * i + 1;
                    if left_i < self.nodes.len() {
                        self.nodes[left_i].pending = match self.nodes[left_i].pending {
                            Some(old_val) => Some(old_val + val),
                            None => Some(val),
                        }
                    }

                    let right_i = 2 * i + 2;
                    if right_i < self.nodes.len() {
                        self.nodes[right_i].pending = match self.nodes[right_i].pending {
                            Some(old_val) => Some(old_val + val),
                            None => Some(val),
                        }
                    }
                }
            }
            None => { /*Nothing to propagate*/ }
        }
    }
}
