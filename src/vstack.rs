#[derive(Debug)]
pub struct VirtualStack {
    items: Vec<usize>,
}

impl VirtualStack {
    pub fn new() -> VirtualStack {
        VirtualStack { items: Vec::new() }
    }

    pub fn push(&mut self, size: usize) -> usize {
        self.items.push(size);
        let offset = self.items.len() * 8;
        offset
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, idx: usize) -> usize {
        self.items[idx].clone()
    }

    pub fn pop(&mut self) {
        self.items.pop();
    }
}
