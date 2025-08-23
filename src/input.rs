use crate::prelude::*;

#[derive(Default)]
pub struct Inputs {
    pub pressed: HashSet<KeyCode>,
    pub just_pressed: HashSet<KeyCode>,
    pub just_released: HashSet<KeyCode>,
}

pub fn input_system(inputs: &mut Inputs) {
    inputs.just_released.clear();
    inputs.just_pressed.clear();
    if event::poll(Duration::from_secs(0)).expect("Poll event")
        && let event::Event::Key(key_event) = event::read().expect("Read event")
    {
        match key_event.kind {
            event::KeyEventKind::Press => {
                if !inputs.pressed.contains(&key_event.code) {
                    inputs.just_pressed.insert(key_event.code);
                }
                inputs.pressed.insert(key_event.code);
            }
            event::KeyEventKind::Release => {
                if inputs.pressed.contains(&key_event.code) {
                    inputs.just_released.insert(key_event.code);
                }
                inputs.pressed.remove(&key_event.code);
            }
            _ => {}
        }
    }
}
