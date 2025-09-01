//! Handles input

use crate::prelude::*;

/// A global resource that contains input states.
#[derive(Default)]
pub struct Inputs {
    /// All the keys that is being pressed.
    pub pressed: HashSet<KeyCode>,
    /// All the keys that has *just* been pressed.
    pub just_pressed: HashSet<KeyCode>,
    /// All the keys that has *just* been released.
    pub just_released: HashSet<KeyCode>,
}

/// Handles terminal input event and store the information inside [`Inputs`] resource.
pub fn input_system(inputs: &mut Inputs) {
    // Clear the inputs from last frame.
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
