use std::{marker::PhantomData, ops::Deref};

use bevy::prelude::Resource;
use kira::clock::ClockHandle;

/// TODO: docs
#[derive(Resource)]
pub struct AudioClock<T> {
    handle: ClockHandle,
    _marker: PhantomData<T>,
}

impl<T> AudioClock<T> {
    pub(crate) fn new(handle: ClockHandle) -> Self {
        Self {
            handle,
            _marker: PhantomData::default(),
        }
    }

    /// TODO: docs
    pub fn handle(&self) -> &ClockHandle {
        &self.handle
    }
}

impl<T> Deref for AudioClock<T> {
    type Target = ClockHandle;

    fn deref(&self) -> &Self::Target {
        self.handle()
    }
}
