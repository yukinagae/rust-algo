use std::collections::VecDeque;

fn main() {
    let mut que = VecDeque::new();
    que.push_back(1);
    que.push_back(2);
    que.push_back(3);
    println!("{:?}", que);
    println!("{:?}", que.front());
    que.pop_front();
    println!("{:?}", que);
    println!("{:?}", que.front());
    que.pop_front();
    println!("{:?}", que);
    println!("{:?}", que.front());
}
