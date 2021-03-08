use crate::{oxi, Synth};

use oxi::{chorus::ChorusMode, synth::chorus::ChorusParams};

impl Synth {
    /**
    Set up the chorus. It should be turned on with Synth::chorus_on().
    If faulty parameters are given, all new settings are discarded.
    Keep in mind, that the needed CPU time is proportional to `nr`.
     */
    pub fn set_chorus_params(
        &mut self,
        nr: u32,
        level: f32,
        speed: f32,
        depth: f32,
        mode: ChorusMode,
    ) {
        self.handle.set_chorus_params(nr, level, speed, depth, mode);
    }

    /**
    Set up the chorus. It should be turned on with Synth::chorus_on().
    If faulty parameters are given, all new settings are discarded.
    Keep in mind, that the needed CPU time is proportional to `nr`.
     */
    pub fn set_chorus(&mut self, params: &ChorusParams) {
        self.handle.set_chorus(params);
    }

    /** Turn on/off the built-in chorus unit */
    pub fn set_chorus_on(&mut self, on: bool) {
        self.handle.set_chorus_on(on);
    }

    /**
    Query the current chorus nr
     */
    pub fn get_chorus_nr(&self) -> u32 {
        self.handle.get_chorus_nr()
    }

    /**
    Query the current chorus level
     */
    pub fn get_chorus_level(&self) -> f32 {
        self.handle.get_chorus_level()
    }

    /**
    Query the current chorus speed (Hz)
     */
    pub fn get_chorus_speed(&self) -> f32 {
        self.handle.get_chorus_speed_hz()
    }

    /**
    Query the current chorus depth (mS)
     */
    pub fn get_chorus_depth(&self) -> f32 {
        self.handle.get_chorus_depth_ms()
    }

    /**
    Query the current chorus mode
     */
    pub fn get_chorus_mode(&self) -> ChorusMode {
        self.handle.get_chorus_mode()
    }

    /**
    Query the current chorus params
     */
    pub fn get_chorus(&self) -> ChorusParams {
        self.handle.get_chorus()
    }
}
