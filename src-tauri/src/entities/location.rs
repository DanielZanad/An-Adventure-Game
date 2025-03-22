use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::helpers::helper::ActionLink;

use super::actions::Actions;

#[derive(Debug)]
pub struct Location {
    name: String,
    actions: ActionLink,
}

impl Location {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            actions: None,
        }
    }

    pub fn add_action(&mut self, action: Arc<Mutex<Actions>>) {
        match &self.actions {
            Some(first) => {
                let mut current = first.clone();
                while let Some(next) = {
                    let current_guard = current.lock().unwrap();
                    current_guard.next.as_ref().map(|next| next.clone())
                } {
                    current = next;
                }
                current.lock().unwrap().next = Some(action.clone());
                action.lock().unwrap().prev = Some(current);
            }
            None => self.actions = Some(action),
        }
    }

    pub fn perform_action(&self, input: &str) -> String {
        let mut current = self.actions.clone();
        while let Some(action) = current {
            let mut action_guard = action.lock().unwrap();
            if action_guard.action_triggers.contains(&input) {
                return action_guard.complete();
            }
            current = action_guard.next.clone()
        }
        return format!("Ação {} não encontrada nesta localização.", input);
    }

    pub fn show_actions(&self) {
        match &self.actions {
            Some(first) => {
                let mut current = first.clone();
                while let Some(next) = {
                    let current_guard = current.lock().unwrap();
                    println!("Nome da ação: {}", current_guard.name);
                    println!("Descrição da ação: {}", current_guard.description);
                    println!("Nome da ação: {:?}", current_guard.action_triggers);
                    println!("Ação está completa?: {}", current_guard.completed);
                    current_guard.next.as_ref().map(|next| next.clone())
                } {
                    current = next;
                }
            }
            None => {}
        }
    }
}
