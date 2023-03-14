fn main() {
    use enum_dispatch_examples::{PushPop, PushPopCollection, Queue, Stack};

    let mut collection = PushPopCollection::<i32>::Stack(Stack::new());
    collection.push(1);
    collection.push(2);
    collection.push(3);

    collection.pop();
    println!("stack: {collection:?}");

    let mut collection = PushPopCollection::<i32>::Queue(Queue::new());
    collection.push(1);
    collection.push(2);
    collection.push(3);

    collection.pop();
    println!("queue: {collection:?}");
}
