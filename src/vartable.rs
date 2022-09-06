use crate::vstack::VirtualStack;

#[derive(Debug, Clone)]
pub struct VarItem {
    name: String,
    stack_offset: usize,
    pub scope_level: u32,
}

impl VarItem {
    pub fn new(name: String, stack_offset: usize, scope_level: u32) -> VarItem {
        VarItem {
            name,
            stack_offset,
            scope_level,
        }
    }
}

#[derive(Debug)]
pub struct VarTable {
    pub items: Vec<VarItem>,
    pub vstack: VirtualStack,
    pub scopes: u32,
}

impl VarTable {
    pub fn new() -> VarTable {
        VarTable {
            items: Vec::new(),
            vstack: VirtualStack::new(),
            scopes: 0,
        }
    }

    pub fn open_scope(&mut self) {
        self.scopes += 1;
    }

    pub fn close_scope(&mut self) {
        self.scopes -= 1;
    }

    pub fn push(&mut self, name: String, size: usize) -> usize {
        let offset = self.vstack.push(size);
        self.items.push(VarItem::new(name, offset, self.scopes));
        offset
    }

    pub fn find(&self, name: &String) -> usize {
        for item in &self.items {
            if item.name == *name {
                return item.stack_offset;
            }
        }
        crate::error::panic(format!("Couldn't find stack offset for variable: {}", name));
    }

    pub fn back(&self) -> VarItem {
        self.items.get(self.items.len() - 1).unwrap().clone()
    }

    pub fn pop(&mut self) {
        self.items.pop();
    }
}
