#[derive(Debug, Clone)]
pub struct StackItem {
    pub size: usize,
    pub signed: Option<bool>,
}

impl StackItem {
    pub fn new(size: usize, signed: Option<bool>) -> StackItem {
        StackItem { size, signed }
    }
}

#[derive(Debug)]
pub struct VirtualStack {
    items: Vec<StackItem>,
}

impl VirtualStack {
    pub fn new() -> VirtualStack {
        VirtualStack { items: Vec::new() }
    }

    pub fn push(&mut self, size: usize, signed: Option<bool>) -> usize {
        self.items.push(StackItem::new(size, signed));
        let offset = self.items.len() * 8;
        offset
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, idx: usize) -> StackItem {
        self.items[idx].clone()
    }

    pub fn pop_item(&mut self) {
        self.items.pop();
    }
}
