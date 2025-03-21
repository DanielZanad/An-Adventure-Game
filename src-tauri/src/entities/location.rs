use std::{cell::RefCell, rc::Rc};

use crate::helpers::helper::ActionLink;

use super::actions::Actions;

#[derive(Debug)]
pub struct Location {
    name: String,
    actions: ActionLink,
}

impl Location {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            actions: None,
        }
    }

    fn add_action(&mut self, action: Rc<RefCell<Actions>>) {
        match &self.actions {
            Some(first) => {
                let mut current = first.clone();
                while let Some(next) = {
                    let current_borrow = current.borrow();
                    current_borrow.next.as_ref().map(|next| next.clone())
                } {
                    current = next;
                }
                current.borrow_mut().next = Some(action.clone());
                action.borrow_mut().prev = Some(current);
            }
            None => self.actions = Some(action),
        }
    }

    fn perform_action(&self, input: &str) -> String {
        let mut current = self.actions.clone();
        while let Some(action) = current {
            if action.borrow().action_triggers.contains(&input.to_string()) {
                return action.borrow_mut().complete();
            }
            current = action.borrow().next.clone()
        }
        return format!("Ação {} não encontrada nesta localização.", input);
    }
}
