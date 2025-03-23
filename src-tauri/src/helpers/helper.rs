use crate::entities::action::Actions;
use std::sync::{Arc, Mutex};

pub type ActionLink = Option<Arc<Mutex<Actions>>>;
