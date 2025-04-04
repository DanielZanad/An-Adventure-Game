use std::sync::{Arc, Mutex};

use serde::Serialize;

use crate::helpers::helper::ActionLink;

use super::{action::Actions, character::Character};

#[derive(Debug, Serialize, Clone)]
pub struct Location {
    pub name: String,
    actions: ActionLink,
    look_around_messages: Vec<String>,
    characters: Vec<Character>,
    pub connections_names: Vec<String>,
}

impl Location {
    pub fn new(
        name: &str,
        look_around_messages: Vec<String>,
        characters: Vec<Character>,
        connections_names: Vec<String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            characters,
            look_around_messages,
            actions: None,
            connections_names,
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

    pub fn perform_action(&mut self, input: &str) -> String {
        let mut current = self.actions.clone();
        while let Some(action) = current {
            let mut action_guard = action.lock().unwrap();
            if action_guard.action_triggers.contains(&input.to_string()) {
                for character in &mut self.characters {
                    character.change_initial(&action_guard.name);
                }
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

    pub fn look_around(&self) -> Vec<String> {
        let mut look_around_messages = vec![];

        for look_around_message in &self.look_around_messages {
            look_around_messages.push(look_around_message.clone());
        }

        look_around_messages
    }
}
