fn main() {
    let mut s = Vec::new();
    s.push(1);
    s.push(2);
    s.push(3);
    println!("{:?}", s.last());
    s.pop();
    println!("{:?}", s.last());
    s.pop();
    println!("{:?}", s.last());
}
