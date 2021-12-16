use std::hash::Hash;

use ahash::AHashMap;
use itertools::Itertools;

/// Input handler for an polling-based game engine.
///
/// Use this when your game engine provides inputs via an poll system.
///
/// At the top of your game loop, you MUST call [`PollingInputHandler::update`]
/// The general logic should look like this:
///
/// ```rust
/// # use puppetmaster::PollingInputHandler;
/// // This is predefined by your game engine
/// #[derive(Clone, Copy, Hash, Eq, PartialEq)]
/// enum Key {
///     Up, Down, Left, Right, Escape,
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
/// let mut input_handler = PollingInputHandler::new_with_controls(vec![
///     (Key::Up, Control::Up),
///     (Key::Down, Control::Down),
///     (Key::Left, Control::Left),
///     (Key::Right, Control::Right),
///     (Key::Escape, Control::Pause),
/// ]);
///
/// # fn get_pressed_keys() -> Vec<Key> { Vec::new() }
/// # struct Player { x: f32 }
/// # impl Player { fn jump(&mut self) {}}
/// # let mut player = Player { x: 0.0 };
/// loop {     
///     // Get all the pressed keys in a manner specific to your game engine.
///     let keys = get_pressed_keys();
///     input_handler.update(keys);
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
pub struct PollingInputHandler<I, C> {
    /// Maps inputs to the controls they activate
    control_config: AHashMap<I, C>,
    /// How long each control has been pressed
    control_time: AHashMap<C, u32>,
}

impl<I: Hash + Eq + Clone, C: Hash + Eq + Clone> PollingInputHandler<I, C> {
    /// Create a new `PollingInputHandler` with no control mapping.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new `PollingInputHandler` with the given mapping of inputs to controls.
    ///
    /// If two entries in the iterator have the same input, the first one will be clobbered
    /// and the second one will remain.
    pub fn new_with_controls(map: impl IntoIterator<Item = (I, C)>) -> Self {
        let control_config = map.into_iter().collect();
        Self {
            control_config,
            control_time: AHashMap::new(),
        }
    }
    /// Manually unpress all inputs.
    ///
    /// Note you should *not* have to call this at the beginning of your loop. (In fact, if you do,
    /// your inputs will never be pressed.)
    pub fn clear_inputs(&mut self) {
        self.control_time.clear();
    }

    /// Update the input handler, giving it the inputs that are currently pressed this frame.
    ///
    /// You MUST CALL THIS FIRST THING in your game loop.
    /// Otherwise things won't get updated correctly.
    pub fn update(&mut self, pressed_inputs: impl IntoIterator<Item = I>) {
        // We want to logical-OR any keypresses into one control.
        // (We collect to a vec because we probably won't be pressing more than 3-4 keys per frame, and I bet the O(n) lookup doesn't
        // get good until then.)
        let pressed_controls = pressed_inputs
            .into_iter()
            .flat_map(|input| {
                let _ = &self;
                self.control_config.get(&input)
            })
            .cloned()
            .collect_vec();
        for ctrl in self.control_config.values() {
            if pressed_controls.contains(ctrl) {
                *self.control_time.entry(ctrl.clone()).or_default() += 1;
            } else {
                self.control_time.insert(ctrl.clone(), 0);
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
}

impl<I, C> Default for PollingInputHandler<I, C> {
    fn default() -> Self {
        Self {
            control_config: AHashMap::new(),
            control_time: AHashMap::new(),
        }
    }
}
