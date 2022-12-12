use std::io;
mod lib;

struct Operation {
    start: i32, // Range start
    end: i32,   // Range end
    val: i32,   // Value to increase the range's elements by
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    // Read n, m and k
    stdin.read_line(&mut buffer).expect("Failed to read line.");
    let tmp: Vec<usize> = buffer
        .split_whitespace()
        .map(|x| x.parse().expect("Not a natural number!"))
        .collect();
    let n: usize = *tmp.first().expect("Missing input parameters.");
    let m: usize = *tmp.get(1).expect("Missing input parameters.");
    let k: usize = *tmp.get(2).expect("Missing input parameters.");

    buffer.clear();

    // Read input array
    stdin.read_line(&mut buffer).expect("Failed to read line.");
    let elems: Vec<i32> = buffer
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();

    // Create a segment tree over the array of elements
    let mut seg_tree = lib::SegmentTree::new(&elems);

    // Read operations
    let mut ops: Vec<Operation> = Vec::with_capacity(m);
    for _i in 0..m {
        buffer.clear();

        stdin.read_line(&mut buffer).expect("Failed to read line.");
        let tmp: Vec<i32> = buffer
            .split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        let start: i32 = *tmp.first().expect("Malformed operation.") - 1;
        let end: i32 = *tmp.get(1).expect("Malformed operation.") - 1;
        let val: i32 = *tmp.get(2).expect("Malformed operation.");

        ops.push(Operation { start, end, val });
    }

    // Difference array initialized to 0
    let mut ops_diff_array: Vec<i32> = vec![0; m + 1];

    // Read queries
    for _i in 0..k {
        buffer.clear();

        stdin.read_line(&mut buffer).expect("Failed to read line.");
        let tmp: Vec<i32> = buffer
            .split_whitespace()
            .map(|x| x.parse().expect("Not a natural number!"))
            .collect();
        let start: i32 = *tmp.first().expect("Malformed query.") - 1;
        let end: i32 = *tmp.get(1).expect("Malformed query.") - 1;

        // Update difference array
        ops_diff_array[start as usize] += 1;
        ops_diff_array[end as usize + 1] -= 1;
    }

    // Create a new array in which to hold the number of times
    //  the ith operation must be repeated, according to the values in
    //  the difference array.
    let mut ops_count: Vec<i32> = Vec::with_capacity(m);
    for i in 0..m {
        let op = &ops[i];
        if i == 0 {
            ops_count.push(ops_diff_array[i]);
        } else {
            ops_count.push(ops_diff_array[i] + ops_count[i - 1]);
        }

        // We can now perform the update just once instead of ops_count[i] times
        seg_tree.range_update(0, op.val * ops_count[i], op.start, op.end);
    }

    // Print result array
    let mut leaves: Vec<i32> = Vec::with_capacity(n);
    seg_tree.get_leaves(0, &mut leaves);
    for leaf in leaves.iter().take(n - 1) {
        print!("{} ", leaf);
    }
    print!("{}", leaves[n - 1]);
    println!();
}
