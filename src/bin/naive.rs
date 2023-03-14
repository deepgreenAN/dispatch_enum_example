use enum_dispatch_examples::{PushPop, Queue, Stack};

#[derive(Debug)]
enum PushPopCollection<T> {
    Stack(Stack<T>),
    Queue(Queue<T>),
}

impl<T> PushPop<T> for PushPopCollection<T> {
    fn push(&mut self, value: T) {
        match self {
            Self::Stack(stack) => stack.push(value),
            Self::Queue(queue) => queue.push(value),
        }
    }
    fn pop(&mut self) -> Option<T> {
        match self {
            Self::Stack(stack) => stack.pop(),
            Self::Queue(queue) => queue.pop(),
        }
    }
    fn is_empty(&self) -> bool {
        match self {
            Self::Stack(stack) => stack.is_empty(),
            Self::Queue(queue) => queue.is_empty(),
        }
    }
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
