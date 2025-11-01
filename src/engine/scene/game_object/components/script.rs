//! A trait describing behavior of a script component in the game engine.
//!
//! Defines contract for game logic scripts that operate on game objects.
//! Each script can apply an action to modify a given `GameObject`.
//! Also requires an associated constructor method with initialization state.

use crate::engine::scene::game_object::GameObject;

/// Trait representing a script that can be attached to a game object.
///
/// Scripts encapsulate game logic that modifies the game object's state.
/// This trait requires the implementation of:
/// - an `action` method performing operations on a mutable reference to `GameObject`
/// - a `new` constructor with an initialization parameter `is_downed`
///
/// Implementors can define custom behavior for initialization and per-frame updates.
pub trait Script {
    /// Apply the script's action on the given mutable game object reference.
    ///
    /// Allows modifying object state, trigger events, or update components.
    fn action(&mut self, game_object: &mut GameObject);

    /// Construct a new instance of the script.
    ///
    /// The `is_downed` parameter allows differentiating initial states or modes.
    /// Implementations can use this to customize setup logic.
    fn new(is_downed: bool) -> Self
    where
        Self: Sized;
}
