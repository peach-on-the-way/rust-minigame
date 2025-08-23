pub use crate::{collision::*, damage::*, draw::*, ecs::*, enemy::*, input::*, player::*, sprite::*, timer::*, ui::*, utils::*};
pub use crossterm::{cursor, event::{self, KeyCode}, execute, queue, style::{self, Color, Stylize}, terminal};
pub use std::{collections::HashSet, io::{self, Stdout, StdoutLock, Write}, time::Duration};

pub type Vec2i32 = (i32, i32);
pub type Vec2usize = (usize, usize);
pub type Health = i32;
