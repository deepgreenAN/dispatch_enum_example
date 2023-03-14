fn main() {
    use enum_dispatch_examples::{PushPop, Queue, Stack};
    let mut collections: Vec<Box<dyn PushPop<i32>>> =
        vec![Box::new(Stack::new()), Box::new(Queue::new())];

    collections[0].push(1);
    collections[0].push(2);
    collections[0].push(3);

    let popped_item = collections[0].pop();
    println!("popped item: {popped_item:?}");

    collections[1].push(1);
    collections[1].push(2);
    collections[1].push(3);

    let popped_item = collections[1].pop();
    println!("popped item: {popped_item:?}");
}
