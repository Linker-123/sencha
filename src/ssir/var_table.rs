use crate::typechecker::TaggedType;

#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    scope_level: usize,
    pub tagged_type: TaggedType,
}

impl Variable {
    pub fn new(name: String, tagged_type: TaggedType) -> Variable {
        Variable {
            name,
            scope_level: 0,
            tagged_type,
        }
    }
}

#[derive(Debug)]
pub struct VarTable {
    variables: Vec<Variable>,
    scope_level: usize,
}

impl VarTable {
    pub fn new() -> VarTable {
        VarTable {
            variables: Vec::new(),
            scope_level: 0,
        }
    }

    pub fn add_var(&mut self, mut var: Variable) {
        var.scope_level = self.scope_level;
        self.variables.push(var);
    }

    pub fn add_scope(&mut self) {
        self.scope_level += 1;
    }

    pub fn end_scope(&mut self) {
        self.variables.retain(|x| x.scope_level < self.scope_level);
        self.scope_level -= 1;
    }

    pub fn get_var(&self, var_name: String) -> Option<Variable> {
        let var = self.variables.iter().rev().find(|x| x.name == var_name);
        if let Some(v) = var {
            Some(v.clone())
        } else {
            None
        }
    }
}
