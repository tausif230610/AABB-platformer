

use godot::prelude::*;


struct Rst;
mod controler;// the controler class
mod playerbehaviour;// the player behaviour
mod godotbox;// the base box which would be moved and responsible for everything.
#[gdextension]
unsafe impl ExtensionLibrary for Rst {}// init stuff
