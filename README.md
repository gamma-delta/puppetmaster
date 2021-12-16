# Puppetmaster

Easy-to-use input handling for writing games.

This crate exposes different kinds of input handlers. Pick which one to use based on how your game engine
provides you with input data.

- `EventInputHandler` for when your game engine uses events
- `PollingInputHandler` for when your game engine provides a set of the currently pressed keys
- `QueryInputHandler` for when your game engine provides a function to call to query the state of a key.

## Inputs vs Controls

This crate makes a distinction between *inputs* and *controls*. 
*Inputs* are the raw keycodes your game engine feeds to you; *controls* are what your game does about it.

So, something like `Key::W` would be an input, predefined by your game engine, 
and you could map it to a `Control::Up` you defined in your game's code.

Multiple inputs can map to the same control, but not vice versa. So, both the W key and the up arrow could
map to `Control::Up`, but you couldn't have the shift key map to both Crouch and Sprint.

---

The crate also includes example code for using each of the input handler types.

All of these are written with GGEZ. I would like to have different game engines to show off Puppetmaster's
flexibility, but unfortunately GGEZ and Macroquad conflict over versions of `alsa`. So, everything's
GGEZ because you can use it with all 3 types of input handler.
