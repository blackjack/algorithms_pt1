mod quickmerge;

fn main() {

    let mut m = quickmerge::QuickMerge::new(10);

    m.union(4, 3);
    m.union(3, 8);
    m.union(6, 5);
    m.union(9, 4);
    m.union(2, 1);

    println!("{}", m);
    println!("connected(8,9): {}", m.connected(8, 9));
    println!("connected(5,4): {}", m.connected(5, 4));

    m.union(5, 0);
    m.union(7, 2);
    m.union(6, 1);
    m.union(7, 3);

    println!("{}", m);

    println!("\n\n{}", m);
}
