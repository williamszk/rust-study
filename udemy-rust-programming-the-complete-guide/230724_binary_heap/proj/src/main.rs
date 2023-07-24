use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");

    // The binary heap will keep the largest value at the top
    let mut bheap = BinaryHeap::new();
    bheap.push(1);
    bheap.push(18);
    bheap.push(20);
    bheap.push(5);
    println!("{:?}", bheap);
    // we can check the value at the top of the heap without removing it from the heap
    println!("{:?}", bheap.peek());
    bheap.pop();
    println!("{:?}", bheap);
    println!("{:?}", bheap.peek());
    bheap.pop();
    println!("{:?}", bheap);
    println!("{:?}", bheap.peek());
    bheap.pop();
    println!("{:?}", bheap);
}
