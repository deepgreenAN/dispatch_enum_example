use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Stack<T>(Vec<T>);

#[derive(Debug, Default)]
pub struct Queue<T>(VecDeque<T>);

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }
}

#[ambassador::delegatable_trait]
#[enum_dispatch::enum_dispatch]
pub trait PushPop<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
}

impl<T> PushPop<T> for Stack<T> {
    fn push(&mut self, value: T) {
        self.0.push(value);
    }
    fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> PushPop<T> for Queue<T> {
    fn push(&mut self, value: T) {
        self.0.push_back(value);
    }
    fn pop(&mut self) -> Option<T> {
        self.0.pop_front()
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// enum_dispatchはここで定義しないといけない．
#[derive(Debug)]
#[enum_dispatch::enum_dispatch(PushPop<X>)]
pub enum PushPopCollection<T> {
    Stack(Stack<T>),
    Queue(Queue<T>),
}
