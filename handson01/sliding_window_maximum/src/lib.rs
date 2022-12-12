use binary_search_tree::BinarySearchTree;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

pub fn brute_force(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = v.len();
    let mut maximums = Vec::with_capacity(n - k + 1);
    for i in 0..(n - k + 1) {
        let current_slice = &v[i..i + k];
        let max_value = *current_slice.iter().max().unwrap();
        maximums.push(max_value);
    }
    maximums
}

pub fn brute_force_idiomatic(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    v.windows(k).map(|w| *w.iter().max().unwrap()).collect()
}

// Window maximum candidate with both value and array position.
#[derive(Eq)]
struct Candidate {
    val: i32,
    pos: usize,
}

impl Ord for Candidate {
    fn cmp(&self, other: &Candidate) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Candidate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Candidate) -> bool {
        self.val == other.val && self.pos == other.pos
    }
}

pub fn heap(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;

    if k < 1 || nums.len() < k {
        panic!("k must be a positive number no larger than nums' length.")
    }

    let mut max_vals: Vec<i32> = Vec::with_capacity(nums.len() - k + 1);
    let mut heap: BinaryHeap<Candidate> = BinaryHeap::new();

    for (i, num) in nums.iter().enumerate() {
        let candidate = Candidate { val: *num, pos: i };

        heap.push(candidate);

        // Once the window is fully formed, remove any
        // root element which is out of it.
        // The first unpopped root element is the current
        // window's maximum.
        if i >= k - 1 {
            while let Some(candidate) = heap.peek() {
                if candidate.pos + k <= i {
                    heap.pop();
                } else {
                    max_vals.push(candidate.val);
                    break;
                }
            }
        }
    }

    max_vals
}

pub fn bst(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;

    if k < 1 || nums.len() < k {
        panic!("k must be a positive number no larger than nums' length.")
    }

    let mut max_vals: Vec<i32> = Vec::with_capacity(nums.len() - k + 1);
    let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();

    for (i, num) in nums.iter().enumerate() {
        tree.insert(*num);

        // Remove the element which is no longer in the window, if any.
        if i >= k {
            tree.remove(&nums[i - k]);
        }

        // Once the window is fully formed, we start registering maxima.
        // Note that the tree will never be empty at this point.
        if i >= k - 1 {
            max_vals.push(*tree.max().unwrap());
        }
    }

    max_vals
}

pub fn linear(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;

    if k < 1 || nums.len() < k {
        panic!("k must be a positive number no larger than nums' length.")
    }

    let mut max_vals: Vec<i32> = Vec::with_capacity(nums.len() - k + 1);
    let mut candidate_queue: VecDeque<i32> = VecDeque::new();

    for (i, num) in nums.iter().enumerate() {
        // Pop the front of the queue if it's no longer in the window.
        if i >= k {
            match candidate_queue.front() {
                Some(val) if *val == nums[i - k] => {
                    candidate_queue.pop_front();
                }
                _ => (),
            }
        }

        insert_candidate(&mut candidate_queue, num);

        // We only start populating the solution array when the window is fully formed.
        // The queue's front is always the maximum for the current window.
        // Note that the queue will never be empty at this point.
        if i >= k - 1 {
            max_vals.push(*candidate_queue.front().unwrap());
        }
    }

    max_vals
}

fn insert_candidate(queue: &mut VecDeque<i32>, val: &i32) {
    // Remove from the tail any element smaller than the current candidate.
    // Doing this, we force the queue's front to contain the largest element.
    while !queue.is_empty() && queue.back().unwrap() < val {
        queue.pop_back();
    }

    queue.push_back(*val);
}

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        nums.push(rng.gen_range(0..i32::MAX));
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idiomatic_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = brute_force_idiomatic(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }
}
