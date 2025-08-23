pub struct Events<T: 'static> {
    events: Vec<T>,
}

impl<T: 'static> Events<T> {
    pub fn push(&mut self, event: T) {
        self.events.push(event);
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

pub struct EventReader<'a, T: 'static> {
    last_read: &'a mut usize,
    events: &'a Events<T>,
}

impl<'a, T: 'static> EventReader<'a, T> {
    pub fn read(&mut self) -> impl Iterator<Item = &'a T> {
        self.events
            .events
            .iter()
            .skip(*self.last_read)
            .inspect(|_| {
                *self.last_read += 1;
            })
    }
}

pub struct EventWriter<'a, T: 'static> {
    events: &'a mut Events<T>,
}

impl<'a, T: 'static> EventWriter<'a, T> {
    pub fn write(&mut self, event: T) {
        self.events.push(event);
    }
}
