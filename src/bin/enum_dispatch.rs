fn main() {
    use enum_dispatch_examples::{PushPop, PushPopCollection, Queue, Stack};

    let mut collections: Vec<PushPopCollection<i32>> = vec![
        PushPopCollection::Stack(Stack::new()),
        PushPopCollection::Queue(Queue::new()),
    ];

    collections[0].push(1);
    collections[0].push(2);
    collections[0].push(3);

    collections[0].pop();
    println!("stack: {:?}", collections[0]);

    collections[1].push(1);
    collections[1].push(2);
    collections[1].push(3);

    collections[1].pop();
    println!("queue: {:?}", collections[1]);
}
