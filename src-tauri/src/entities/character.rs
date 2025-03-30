// (DialogTrigger, Dialog)
type DialogTrigger = (String, String);

#[derive(Debug)]
pub struct Character {
    name: String,
    dialogs: Vec<Dialog>,
    initial_dialog: String,
    initial_dialogs_change_triggers: Vec<DialogTrigger>,
}
impl Character {
    pub fn new(
        name: &str,
        dialogs: Vec<Dialog>,
        initial_dialog: &str,
        initial_dialogs_change_triggers: Vec<DialogTrigger>,
    ) -> Self {
        Character {
            name: name.to_string(),
            initial_dialogs_change_triggers,
            initial_dialog: initial_dialog.to_string(),
            dialogs,
        }
    }

    pub fn change_initial(&mut self, action_name: &String) {
        for dialog_change in &mut self.initial_dialogs_change_triggers {
            if dialog_change.0.eq(action_name) {
                self.initial_dialog = dialog_change.1.clone();
            }
        }
    }
}

#[derive(Debug)]
pub struct Dialog {
    question: String,
    answer: String,
}

impl Dialog {
    pub fn new(question: &str, answer: &str) -> Self {
        Dialog {
            question: question.to_string(),
            answer: answer.to_string(),
        }
    }
}
