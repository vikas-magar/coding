use std::collections::HashMap;

fn first_duplicate_value(v: Vec<i32>) -> i32 {
    /* getting element in hashmap is O(n) Operation*/
    let mut store: HashMap<i32, i32> = HashMap::new();

    for ele in v.iter() {
        match store.get(ele) {
            Some(ele) => {
                return *ele;
            }
            None => {
                store.insert(*ele, *ele);
            }
        };
    }
    return -1;
}
pub fn run() {
    let v = vec![2, 1, 5, 2, 3, 3, 4];
    println!("{:?}", first_duplicate_value(v));
}
