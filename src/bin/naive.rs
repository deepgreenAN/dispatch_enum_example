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
