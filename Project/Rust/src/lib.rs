
use godot::prelude::*;
struct Glue;
mod bounding_box;
#[gdextension]
unsafe impl  ExtensionLibrary for  Glue{}