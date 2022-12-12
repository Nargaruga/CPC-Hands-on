use std::cmp;

#[derive(Copy, Clone, Debug)]
pub struct Data {
    min: i32,
    max: i32,
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

#[derive(Debug)]
pub struct SegmentTree {
    nodes: Vec<Node>,
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
                data: Data {
                    // Fill with dummy data
                    min: i32::MAX,
                    max: i32::MIN,
                },
                pending: None
            };
            tree_size
        ];

        SegmentTree::build_tree(&mut nodes, 0, data, 0, (data.len() - 1) as i32);

        Self { nodes }
    }

    /**
     * Substitutes the elements of the <start, end> (inclusive) range
     *  with the minimum between themselves and val.
     */
    pub fn range_update(&mut self, i: usize, val: i32, start: i32, end: i32) -> Data {
        if i >= self.nodes.len() {
            panic!("Index out of bounds.");
        }

        if start > self.nodes[i].range.end || end < self.nodes[i].range.start {
            //No overlap
            return self.nodes[i].data;
        }

        self.propagate_pending_update(i);

        if self.nodes[i].range.start >= start && self.nodes[i].range.end <= end {
            // Total overlap
            if self.nodes[i].range.start == self.nodes[i].range.end {
                // Update current node and propagate to immediate children
                self.nodes[i].pending = Some(val);
                self.propagate_pending_update(i);
                return self.nodes[i].data;
            }
        }

        // Partial overlap
        let left: Data = self.range_update(2 * i + 1, val, start, end);
        let right: Data = self.range_update(2 * i + 2, val, start, end);

        self.nodes[i].data = SegmentTree::combine_data(left, right);

        self.nodes[i].data
    }

    /**
     * Returns the maximum element in the <start, end> (inclusive) range.
     */
    pub fn range_max(&mut self, i: usize, start: i32, end: i32) -> i32 {
        if i >= self.nodes.len() {
            panic!("Index out of bounds.")
        }

        if start > self.nodes[i].range.end || end < self.nodes[i].range.start {
            //No overlap, return dummy data
            return i32::MIN;
        }

        self.propagate_pending_update(i);

        if start <= self.nodes[i].range.start && end >= self.nodes[i].range.end {
            // Total overlap
            return self.nodes[i].data.max;
        }

        //Partial overlap
        let left: i32 = self.range_max(2 * i + 1, start, end);
        let right: i32 = self.range_max(2 * i + 2, start, end);

        cmp::max(left, right)
    }

    /**
     * Recursively build the segment tree
     */
    fn build_tree(nodes: &mut Vec<Node>, i: usize, data: &Vec<i32>, start: i32, end: i32) -> Data {
        if i >= nodes.len() || start == -1 || end == -1 {
            panic!("Invalid parameters.");
        }

        // Leaf
        if start == end {
            nodes[i] = Node {
                data: Data {
                    //Leaves have min == max
                    min: data[start as usize],
                    max: data[start as usize],
                },
                range: Range { start, end },
                pending: None,
            };

            return nodes[i].data;
        }

        //Non-leaf node
        let m: i32 = ((start as f64 + end as f64) / 2.0).floor() as i32;
        let left = SegmentTree::build_tree(nodes, 2 * i + 1, data, start, m);
        let right = SegmentTree::build_tree(nodes, 2 * i + 2, data, m + 1, end);

        // Create and insert the node
        nodes[i] = Node {
            data: SegmentTree::combine_data(left, right),
            range: Range { start, end },
            pending: None,
        };

        nodes[i].data
    }

    /**
     * Combines two Data objects a and b into one.
     */
    fn combine_data(a: Data, b: Data) -> Data {
        Data {
            min: cmp::min(a.min, b.min),
            max: cmp::max(a.max, b.max),
        }
    }

    /**
     * Applies any pending update on node i and propagates
     * it to the node's children.
     */
    fn propagate_pending_update(&mut self, i: usize) {
        if i >= self.nodes.len() {
            panic!("Index out of bounds.");
        }

        let mut node = &mut self.nodes[i];

        match node.pending {
            Some(val) => {
                node.pending = None;

                // Update ith node
                node.data.min = cmp::min(node.data.min, val);
                node.data.max = cmp::min(node.data.max, val);

                //Set pending update on children
                let left_i = 2 * i + 1;
                if left_i < self.nodes.len() {
                    self.nodes[left_i].pending = match self.nodes[left_i].pending {
                        Some(old_val) => Some(cmp::min(old_val, val)),
                        None => Some(val),
                    }
                }

                let right_i = 2 * i + 2;
                if right_i < self.nodes.len() {
                    self.nodes[right_i].pending = match self.nodes[right_i].pending {
                        Some(old_val) => Some(cmp::min(old_val, val)),
                        None => Some(val),
                    }
                }
            }
            None => { /*Nothing to propagate*/ }
        }
    }
}
