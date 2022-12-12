use std::io;
mod lib;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    // Read n and m
    stdin.read_line(&mut buffer).expect("Failed to read line.");
    let tmp: Vec<i32> = buffer
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    //let n: i32 = *tmp.get(0).expect("Missing input parameters."); // Unnecessary
    let m: i32 = *tmp.get(1).expect("Missing input parameters.");

    buffer.clear();

    // Read input array
    stdin.read_line(&mut buffer).expect("Failed to read line.");
    let elems: Vec<i32> = buffer
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();

    // Create a segment tree over the array of elements
    let mut seg_tree = lib::SegmentTree::new(&elems);

    // Read queries
    for _i in 0..m {
        buffer.clear();

        stdin.read_line(&mut buffer).expect("Failed to read line.");
        let tmp: Vec<i32> = buffer
            .split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        let query_type: i32 = *tmp.first().expect("Malformed query.");
        let left: i32 = *tmp.get(1).expect("Malformed query.") - 1;
        let right: i32 = *tmp.get(2).expect("Malformed query.") - 1;

        match query_type {
            0 => {
                let val: i32 = *tmp.get(3).expect("Malformed query.");
                seg_tree.range_update(0, val, left, right);
            }
            1 => {
                println!("{}", seg_tree.range_max(0, left, right));
            }
            _ => panic!("Unrecognized query type."),
        }
    }
}
