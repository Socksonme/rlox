use ghost_cell::{GhostCell, GhostToken};

pub struct Entry<'brand, 'prev, T> {
    pub data: GhostCell<'brand, T>,
    pub prev: Option<&'prev Entry<'brand, 'prev, T>>,
}

impl<'brand, 'prev, T> Entry<'brand, 'prev, T> {
    pub fn new<'prev_prev: 'prev>(data: T) -> Self {
        Self {
            data: GhostCell::new(data),
            prev: None,
        }
    }
}

pub struct RefChain<'brand, 'prev, T> {
    pub entry: Entry<'brand, 'prev, T>,
    token: &'prev mut GhostToken<'brand>,
}

impl<'brand, 'prev, T> RefChain<'brand, 'prev, T> {
    pub fn new<'prev_prev: 'prev>(
        data: T,
        token: &'prev mut GhostToken<'brand>
    ) -> Self {
        Self {
            entry: Entry::new(data),
            token,
        }
    }
    pub fn with_prev<'prev_prev: 'prev>(
        prev: &'prev mut RefChain<'brand, 'prev_prev, T>,
        data: T,
    ) -> Self {
        let entry = Entry {
            data: GhostCell::new(data),
            prev: Some(&prev.entry),
        };
        Self {
            entry,
            token: &mut *prev.token,
        }
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.entry.data.get_mut()
    }
    pub fn get(&self) -> &T {
        self.entry.data.borrow(self.token)
    }
    pub fn nth_mut(&mut self, mut index: usize) -> Option<&mut T> {
        let mut current = &self.entry;
        while index != 0 {
            current = current.prev?;
            index -= 1;
        }
        Some(current.data.borrow_mut(self.token))
    }
    pub fn split(&mut self) -> (&mut GhostToken<'brand>, &Entry<'brand, 'prev, T>) {
        (&mut self.token, &self.entry)
    }
}