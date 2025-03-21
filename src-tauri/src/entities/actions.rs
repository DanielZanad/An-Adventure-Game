use std::{cell::RefCell, rc::Rc};

use crate::helpers::helper::ActionLink;
#[derive(Debug)]
pub struct Actions {
    name: String,
    pub action_triggers: Vec<String>,
    description: String,
    success_message: String,
    fail_message: Option<String>,
    completed: bool,
    pub prev: ActionLink,
    pub next: ActionLink,
}

impl Actions {
    fn new(
        name: &str,
        action_triggers: Vec<String>,
        description: &str,
        success_message: &str,
        fail_message: Option<String>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
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
            if !prev.borrow().completed {
                return format!(
                    "Você precisa completar {:?} antes de {}",
                    prev.borrow(),
                    if let Some(fail_message) = &self.fail_message {
                        return fail_message.clone();
                    } else {
                        String::new()
                    }
                );
            }
        }
        self.completed = true;
        self.success_message.clone()
    }
}
