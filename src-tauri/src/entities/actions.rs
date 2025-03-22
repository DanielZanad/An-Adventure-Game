use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::helpers::helper::ActionLink;
#[derive(Debug)]
pub struct Actions {
    pub name: String,
    pub action_triggers: Vec<&'static str>,
    pub description: String,
    pub success_message: String,
    pub fail_message: Option<String>,
    pub completed: bool,
    pub prev: ActionLink,
    pub next: ActionLink,
}

impl Actions {
    pub fn new(
        name: &str,
        action_triggers: Vec<&'static str>,
        description: &str,
        success_message: &str,
        fail_message: Option<String>,
    ) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            name: name.to_string(),
            action_triggers,
            description: description.to_string(),
            success_message: success_message.to_string(),
            fail_message,
            completed: false,
            prev: None,
            next: None,
        }))
    }

    pub fn complete(&mut self) -> String {
        if let Some(prev) = &self.prev {
            if let Ok(prev) = prev.lock() {
                if !prev.completed {
                    if let Some(fail_message) = &self.fail_message {
                        return fail_message.clone();
                    } else {
                        return String::new();
                    }
                }
            }
        }
        self.completed = true;
        self.success_message.clone()
    }
}
