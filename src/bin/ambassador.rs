use ambassador::Delegate;
use enum_dispatch_examples::{ambassador_impl_PushPop, PushPop, Queue, Stack};

#[derive(Debug, Delegate)]
#[delegate(PushPop<X>, generics = "X")]
enum PushPopCollection<T> {
    Stack(Stack<T>),
    Queue(Queue<T>),
}

fn main() {
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
