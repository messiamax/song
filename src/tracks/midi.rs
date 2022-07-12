use crate::{
    auto, effects, instruments,
    time::{self, TimeManager},
    wave,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Copy)]
pub struct Pitch {
    value: u8,
}
impl Pitch {
    pub fn new(value: u8) -> Option<Self> {
        if value < 0x80 {
            Some(Self { value })
        } else {
            None
        }
    }

    pub fn get(&self) -> u8 {
        self.value
    }

    pub fn new_unchecked(value: u8) -> Self {
        Self { value }
    }

    pub fn get_freq(&self) -> f64 {
        440.0 * 2.0_f64.powf((self.value as f64 - 69.0) / 12.0)
    }
}

#[derive(Clone, Copy)]
pub struct Note {
    pub pitch: Pitch,
    pub on: time::TimeStamp,
    pub off: time::TimeStamp,
    pub velocity: auto::CtrlVal,
}

pub struct MidiTrack<'a, W: wave::Wave> {
    pub name: String,
    pub instrument: Box<dyn instruments::MidiInstrument<W>>,
    pub gain: f64,
    pub effects: effects::EffectNode<W>,
    pub control_panel: effects::CtrlPanel<'a>,
    pub notes: Vec<Note>,
    pub automation_manager: Rc<RefCell<auto::AutomationManager>>,
    pub time_manager: Rc<RefCell<time::TimeManager>>,
}

impl<W: wave::Wave> time::TimeKeeper for MidiTrack<'_, W> {
    fn set_time_manager(&mut self, time_manager: Rc<RefCell<time::TimeManager>>) {
        self.instrument.set_time_manager(Rc::clone(&time_manager));
        self.effects.set_time_manager(Rc::clone(&time_manager));
        self.control_panel
            .set_time_manager(Rc::clone(&time_manager));
        self.automation_manager
            .borrow_mut()
            .set_time_manager(Rc::clone(&time_manager));
        self.time_manager = Rc::clone(&time_manager)
    }
}

impl<W: wave::Wave> MidiTrack<'_, W> {
    pub fn set_automation_manager(&mut self) {
        self.instrument
            .set_automation_manager(Rc::clone(&self.automation_manager));
    }
}

impl<'a, W: 'static + wave::Wave> MidiTrack<'a, W> {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            instrument: Box::new(instruments::EmptyInstrument::<W>::new()),
            gain: 1.0,
            effects: effects::EffectNode::Bypass,
            control_panel: effects::CtrlPanel::Bypass,
            automation_manager: Rc::new(RefCell::new(auto::AutomationManager::new())),
            notes: Vec::new(),
            time_manager: Rc::new(RefCell::new(time::TimeManager::default())),
        }
    }
    pub fn play(&self) -> W {
        let mut wave = self.instrument.play_notes(&self.notes);
        self.effects.apply(
            &mut wave,
            &self.control_panel,
            self.time_manager.borrow().zero(),
        );
        wave.scale(self.gain);
        wave
    }

    pub fn from_instrument(instrument: Box<dyn instruments::MidiInstrument<W>>) -> Self {
        let automation = Rc::new(RefCell::new(auto::AutomationManager::new()));
        let track = Self {
            name: String::from(instrument.name()),
            instrument,
            gain: 1.0,
            effects: effects::EffectNode::<W>::Bypass,
            control_panel: effects::CtrlPanel::Bypass,
            automation_manager: automation,
            notes: Vec::new(),
            time_manager: Rc::new(RefCell::new(TimeManager::default())),
        };
        track
    }
}

impl<'a, W: 'static + wave::Wave> Default for MidiTrack<'a, W> {
    fn default() -> Self {
        Self::new()
    }
}