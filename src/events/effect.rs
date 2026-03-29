use crate::events::app_event::ActionEvent;

pub enum Effect {
    Action(ActionEvent)
}