fn main() {
    use enum_dispatch_examples::{PushPop, Queue, Stack};
    let mut collection: Box<dyn PushPop<i32>> = Box::new(Stack::new());
    collection.push(1);
    collection.push(2);
    collection.push(3);

    let popped_item = collection.pop();
    println!("popped item: {popped_item:?}");

    let mut collection: Box<dyn PushPop<i32>> = Box::new(Queue::new());
    collection.push(1);
    collection.push(2);
    collection.push(3);

    let popped_item = collection.pop();
    println!("popped item: {popped_item:?}");
}
