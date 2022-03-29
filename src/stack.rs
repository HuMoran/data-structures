#[derive(Debug, Default)]
pub struct Stack<T> {
    top: usize,
    data: Vec<T>,
}
impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.data.pop()
    }
    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1)
    }
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
    pub fn size(&self) -> usize {
        self.top
    }
}

// To repeat `derive(Default)`
// impl <T>Default for Stack<T> {
//     fn default() -> Self {
//         Stack { top: 0, data: Vec::new() }
//     }
// }
