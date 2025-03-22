use crate::entities::actions::Actions;
use std::sync::{Arc, Mutex};

pub type ActionLink = Option<Arc<Mutex<Actions>>>;
