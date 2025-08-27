use bevy::{platform::collections::HashSet, prelude::*};

#[derive(Resource)]
pub struct PlayerInventory { pub stacks: Vec<ObjectStack> }

#[derive(Resource)]
pub struct SelectedEntities { pub entities: HashSet<Entity> }

#[derive(Resource)]
pub struct CanPlayerBuild { pub enabled: bool  } // player pointing at ui entity

// playerinventory / uiinventoryslot
#[derive(Debug, Component)]
pub struct ObjectStack {
    pub item: Object,
    pub total: i32,
    pub assigned: bool, // ui uses this to bind it to the ui
    pub related_entity: Entity
}

#[derive(Debug, Resource)]
pub struct ObjectSelected {
    pub object: Object,
    pub ui_entity: Entity
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum Object {
    None,
    Action,
    Worker
}

pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app.insert_resource(CanPlayerBuild { enabled: false });
        app.insert_resource(PlayerInventory { stacks: Vec::new() });
        app.insert_resource(SelectedEntities { entities: HashSet::new() });
        app.insert_resource(ObjectSelected { object: Object::Worker, ui_entity: Entity::from_raw(0)});

        app.add_systems(Startup, setup);
    }
}

fn setup(mut inv: ResMut<PlayerInventory>) {
    inv.stacks.push(ObjectStack { item: Object::Worker, total:9, assigned:false, related_entity:Entity::from_raw(0) });
}