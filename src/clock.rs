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

    /// How much time has passed since the start of the clock.
    pub fn elapsed_time(&self) -> f64 {
        self.handle.time().ticks as f64 + self.fractional_position()
    }

    /// How many ticks have passed since the start of the clock.
    pub fn elapsed_ticks(&self) -> u64 {
        self.handle.time().ticks
    }
}

impl<T> Deref for AudioClock<T> {
    type Target = ClockHandle;

    fn deref(&self) -> &Self::Target {
        self.handle()
    }
}
