use bevy::prelude::*;

pub struct TopDownPlugin;

impl Plugin for TopDownPlugin {
    fn build(&self, _app: &mut AppBuilder) {

    }
}

impl Default for TopDownPlugin {
    fn default() -> Self {
        TopDownPlugin
    }
}