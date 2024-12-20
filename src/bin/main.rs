use coding_lib::Solution;

fn main() {
    let mut lc = vec![0; 5];
    println!("len {} capacity {}", lc.len(), lc.capacity());
    lc.push(3);
    println!("len {} capacity {}", lc.len(), lc.capacity());
}
