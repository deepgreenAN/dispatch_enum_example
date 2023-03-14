use ambassador::Delegate;
use enum_dispatch_examples::{ambassador_impl_PushPop, PushPop, Queue, Stack};

#[derive(Debug, Delegate)]
#[delegate(PushPop<X>, generics = "X")]
enum PushPopCollection<T> {
    Stack(Stack<T>),
    Queue(Queue<T>),
}

fn main() {
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
