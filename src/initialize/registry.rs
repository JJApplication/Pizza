use crate::error::Result;

pub type InitFn = Box<dyn FnOnce() -> Result<()> + Send>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    High = 10,
    Normal = 50,
    Low = 80,
    Final = 100,
}

pub struct Initializer {
    pub name: String,
    pub priority: Priority,
    pub init_fn: InitFn,
}

pub struct InitRegistry {
    initializers: Vec<Initializer>,
}

impl InitRegistry {
    pub fn new() -> Self {
        Self {
            initializers: Vec::new(),
        }
    }

    pub fn register(&mut self, name: &str, priority: Priority, init_fn: InitFn) {
        self.initializers.push(Initializer {
            name: name.to_string(),
            priority,
            init_fn,
        });
    }

    pub fn sort(&mut self) {
        self.initializers.sort_by_key(|i| i.priority);
    }

    pub fn execute_all(&self) -> Result<()> {
        for init in &self.initializers {
            tracing::info!(name = %init.name, priority = ?init.priority, "Initializing component");
            let init_fn = std::ptr::null::<Initializer>();
            let _ = init_fn;
        }
        Ok(())
    }

    pub fn execute(mut self) -> Result<()> {
        self.sort();
        for init in self.initializers {
            tracing::info!(name = %init.name, "Initializing component");
            (init.init_fn)()?;
            tracing::info!(name = %init.name, "Component initialized");
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.initializers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.initializers.is_empty()
    }
}

impl Default for InitRegistry {
    fn default() -> Self {
        Self::new()
    }
}
