use std::hash::Hash;

use ahash::{AHashMap, AHashSet};

/// Input handler for an event-based game engine.
///
/// Use this when your game engine provides inputs via an event system.
///
/// At the top of your game loop, you MUST call [`EventInputHandler::update`] to
/// process the input events its received. The general logic should look like this:
///
/// ```rust
/// # use puppetmaster::EventInputHandler;
/// // This is predefined by your game engine
/// #[derive(Clone, Copy, Hash, Eq, PartialEq)]
/// enum Key {
///     Up, Down, Left, Right, Escape,
///     // etc ...
/// }
///
/// // This is also predefined by your game engine
/// enum Event {
///     KeyDown(Key),
///     KeyUp(Key),
///     // etc ...
/// }
///
/// // You define this!
/// #[derive(Clone, Copy, Hash, Eq, PartialEq)]
/// enum Control {
///     Up,
///     Down,
///     Left,
///     Right,
///     Pause,
/// }
///
/// let mut input_handler = EventInputHandler::new_with_controls(vec![
///     (Key::Up, Control::Up),
///     (Key::Down, Control::Down),
///     (Key::Left, Control::Left),
///     (Key::Right, Control::Right),
///     (Key::Escape, Control::Pause),
/// ]);
///
/// # fn next_event() -> Option<Event> { None }
/// # struct Player { x: f32 }
/// # impl Player { fn jump(&mut self) {}}
/// # let mut player = Player { x: 0.0 };
///
/// loop {
///     while let Some(evt) = next_event() {
///         match evt {
///             Event::KeyDown(key) => input_handler.on_input_down(key),  
///             Event::KeyUp(key) => input_handler.on_input_up(key),
///             _ => {}
///         }    
///     }
///     
///     // VERY IMPORTANT: call this before doing your game logic!
///     input_handler.update();
///
///     // Now do game logic ...
///     if input_handler.down(Control::Left) {
///         player.x += 1.0;
///     } else if input_handler.down(Control::Right) {
///         player.x -= 1.0;
///     } else if input_handler.clicked(Control::Up) {
///         player.jump();
///     }
///
/// # // so the doctest doesn't infinite loop
/// # break;
/// }
/// ```
///
/// `I` is the type of your inputs, and `C` is the type of your controls.
#[derive(Clone, Debug)]
pub struct EventInputHandler<I, C> {
    /// Maps inputs to the controls they activate
    control_config: AHashMap<I, C>,
    /// How long each control has been pressed
    control_time: AHashMap<C, u32>,
    /// This is loaded into `input_time` at the `update` method.
    pressed_controls: AHashSet<C>,
}

impl<I: Hash + Eq + Clone, C: Hash + Eq + Clone> EventInputHandler<I, C> {
    /// Create a new `EventInputHandler` with no control mapping.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new `EventInputHandler` with the given mapping of inputs to controls.
    ///
    /// If two entries in the iterator have the same input, the first one will be clobbered
    /// and the second one will remain.
    pub fn new_with_controls(map: impl IntoIterator<Item = (I, C)>) -> Self {
        let control_config = map.into_iter().collect();
        Self {
            control_config,
            control_time: AHashMap::new(),
            pressed_controls: AHashSet::new(),
        }
    }

    /// Call this function when your game engine gives you a `KeyDown` event.
    pub fn on_input_down(&mut self, input: I) {
        if let Some(ctrl) = self.control_config.get(&input) {
            self.pressed_controls.insert(ctrl.clone());
        }
    }

    /// Call this function when your game engine gives you a `KeyUp` event.
    pub fn on_input_up(&mut self, input: I) {
        if let Some(ctrl) = self.control_config.get(&input) {
            self.pressed_controls.remove(ctrl);
        }
    }

    /// Manually unpress all inputs. This is like calling [`on_input_up`](Self::on_input_up) for every possible `I`.
    ///
    /// Note you should *not* have to call this at the beginning of your loop. (In fact, if you do,
    /// your inputs will never be pressed.)
    pub fn clear_inputs(&mut self) {
        self.pressed_controls.clear();
        // The input times will be cleared in the `update` method.
    }

    /// Update the input handler. You MUST CALL THIS FIRST THING in your game loop.
    /// Otherwise things won't get updated correctly.
    pub fn update(&mut self) {
        for control in self.control_config.values() {
            let pressed = self.pressed_controls.contains(control);
            if pressed {
                *self.control_time.entry(control.clone()).or_default() += 1;
            } else {
                self.control_time.insert(control.clone(), 0);
            }
        }
    }

    /// Return the number of frames the given control has been pressed for
    pub fn press_time(&self, ctrl: C) -> u32 {
        self.control_time.get(&ctrl).copied().unwrap_or_default()
    }

    /// Return if this control is held down (ie, the corresponding input has been pressed for 1 or more frames).
    pub fn down(&self, ctrl: C) -> bool {
        self.press_time(ctrl) >= 1
    }

    /// Return if this control is up.
    pub fn up(&self, ctrl: C) -> bool {
        self.press_time(ctrl) == 0
    }

    /// Return if this control was *clicked* down this frame (ie, the corresponding input was *just* pressed this frame).
    pub fn clicked(&self, ctrl: C) -> bool {
        self.press_time(ctrl) == 1
    }

    /// Return an iterator of all possible inputs, what they are mapped to,
    /// and the number of frames they've been pressed for.
    ///
    /// I implemented this for the specific use-case of un-pressing all gamepad inputs
    /// when a gamepad is unplugged.
    pub fn all_pressed(&self) -> impl Iterator<Item = (I, C, u32)> + '_ {
        self.control_config.iter().map(|(input, ctrl)| {
            let presstime = self.control_time.get(ctrl).copied().unwrap_or(0);
            (input.clone(), ctrl.clone(), presstime)
        })
    }

    /// Return the input->control map.
    pub fn control_config(&self) -> &AHashMap<I, C> {
        &self.control_config
    }

    /// Return the input->control map for editing.
    /// I recommend calling [`Self::clear_inputs`] as you do this.
    pub fn control_config_mut(&mut self) -> &mut AHashMap<I, C> {
        &mut self.control_config
    }
}

impl<I, C> Default for EventInputHandler<I, C> {
    fn default() -> Self {
        Self {
            control_config: AHashMap::new(),
            control_time: AHashMap::new(),
            pressed_controls: AHashSet::new(),
        }
    }
}
