use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

pub struct WatcherGuard<'a, 'w, T> {
    watcher: &'a mut Watcher<'w, T>,
}

impl<'a, 'w, T> Deref for WatcherGuard<'a, 'w, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.watcher.value
    }
}
impl<'a, 'w, T> DerefMut for WatcherGuard<'a, 'w, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.watcher.value
    }
}
impl<'a, 'w, T> Borrow<T> for WatcherGuard<'a, 'w, T> {
    fn borrow(&self) -> &T {
        self
    }
}
impl<'a, 'w, T> BorrowMut<T> for WatcherGuard<'a, 'w, T> {
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

impl<'a, 'w, T> Drop for WatcherGuard<'a, 'w, T> {
    fn drop(&mut self) {
        for f in self.watcher.callbacks.iter() {
            f(self.watcher);
        }
    }
}

pub struct Watcher<'w, T> {
    value: T,
    callbacks: Vec<Box<dyn Fn(&T) + 'w>>,
}

impl<'w, T> Watcher<'w, T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            callbacks: Vec::new(),
        }
    }

    pub fn connect_notify(&mut self, f: impl Fn(&T) + 'w) {
        self.callbacks.push(Box::new(f));
    }

    pub fn borrow_mut(&mut self) -> WatcherGuard<'_, 'w, T> {
        WatcherGuard { watcher: self }
    }
}

impl<'w, T: Default> Default for Watcher<'w, T> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            callbacks: Default::default(),
        }
    }
}

impl<'w, T> Borrow<T> for Watcher<'w, T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<'w, T> Deref for Watcher<'w, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
