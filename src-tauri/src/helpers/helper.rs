use crate::entities::actions::Actions;
use std::{cell::RefCell, rc::Rc};

pub type ActionLink = Option<Rc<RefCell<Actions>>>;

fn main() {}
