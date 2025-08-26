use bevy::prelude::*;

use crate::player::{CanPlayerBuild, Object, ObjectSelected, ObjectStack, PlayerInventory};

#[derive(Debug, Component)]
struct UiInventorySlot { slot: ObjectStack }

#[derive(Debug, Component)]
struct UiSlot;

pub struct GameUi;

impl Plugin for GameUi {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

        app.add_systems(Update, (can_player_interact, ui_slot_selection, ui_load_items, log_slots));        
    }
}

const WINDOWRESOLUTION: (f32, f32) = (800.0, 600.0);

fn setup(mut commands: Commands) {
    commands.spawn(
    Node {width:Val::Percent(100.),height:Val::Percent(100.),display:Display::Flex,flex_direction:FlexDirection::Column,..default()}, 
    ).with_children(|main| {
        main.spawn(
            Node {
                width:Val::Percent(100.0),
                height:Val::Percent(5.0),
                display:Display::Flex,
                flex_direction:FlexDirection::Row,
                align_content:AlignContent::FlexStart,
                ..default()
            },
        ).with_children(|top_menu| {
            top_menu.spawn((
                Node {
                    width:Val::Percent(100.0),
                    height:Val::Px(18.0),
                    display:Display::Flex,
                    flex_direction:FlexDirection::Row,
                    align_content:AlignContent::FlexStart,
                    ..default()
                }, 
                BackgroundColor(Color::srgb(0.27, 0.27, 0.27)),
                Button
            ));
        });

        main.spawn(
            Node {
                width:Val::Percent(100.0),
                height:Val::Percent(95.0),
                display:Display::Flex,
                flex_direction:FlexDirection::Column,
                ..default()
            },
        ).with_children(|game_screen| {
            game_screen.spawn(
                Node {
                    width:Val::Percent(100.0),
                    height:Val::Percent(90.0),
                    display:Display::Flex,
                    flex_direction:FlexDirection::Row,
                    ..default()
                }
            );

            game_screen.spawn(
                Node {
                    width:Val::Percent(100.0),
                    height:Val::Percent(10.0),
                    display:Display::Flex,
                    flex_direction:FlexDirection::Row,
                    justify_content:JustifyContent::Center,
                    align_items:AlignItems::Center,
                    ..default()
                }
            ).with_children(|access_bar|{
                access_bar.spawn((
                    Node {
                        width:Val::Px(320.0),
                        height:Val::Px(48.0),
                        display:Display::Flex,
                        column_gap:Val::Px(4.),
                        flex_wrap:FlexWrap::Wrap,
                        align_items:AlignItems::Center,
                        flex_direction:FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },BackgroundColor(Color::srgb(0.4,0.4,0.90)),
                    Button
                )).with_children(|object_slots|{
                    for _ in 0..6 {
                        object_slots.spawn(build_item_slot());
                    }
                });
            });
        });
    });
}

fn build_item_slot() -> impl Bundle {
    (
        Node {
            width:Val::Px(40.0),
            height:Val::Px(40.0),
            display:Display::Flex,
            flex_direction:FlexDirection::Row,
            ..default()
        },
        UiInventorySlot { slot: ObjectStack { item:Object::None, total:0, assigned:false, related_entity:Entity::from_raw(0) }},
        BackgroundColor(Color::srgb(0.92, 0.92, 0.92)),
        Button,
        UiSlot
    )
}

fn can_player_interact(mut can_build: ResMut<CanPlayerBuild>, interactions: Query<&Interaction,With<Button>>) {
    let interaction = interactions.iter().any(|i| *i != Interaction::None);
    can_build.enabled = !interaction;
}

fn ui_slot_selection(mut item_selected: ResMut<ObjectSelected>,inventory_slots: Query<(&Interaction,&UiInventorySlot,Entity),(With<UiSlot>, Changed<Interaction>)>) {
    for (interaction, ui_slot, entity) in &inventory_slots {
        if *interaction == Interaction::Pressed { (item_selected.object, item_selected.ui_entity) = (ui_slot.slot.item, entity); println!("{:?}", item_selected); }
    }
}

// sync the inventory items and the ui
fn ui_load_items(mut ui_inventory_slots: Query<(&mut UiInventorySlot, Entity), With<UiSlot>>,mut player_inventory: ResMut<PlayerInventory>) {
    let Some(item_not_assigned) = player_inventory.stacks.iter_mut().find(|i|!i.assigned) else { return; };

    if let Some(mut slot) = ui_inventory_slots.iter_mut().find(|s| s.0.slot.item == Object::None) {
        slot.0.slot.assigned = true;
        slot.0.slot.item = item_not_assigned.item;
        slot.0.slot.total = item_not_assigned.total;

        item_not_assigned.assigned = true; // flag to not assign it again!
        item_not_assigned.related_entity = slot.1; // assign the entity id of the ui slot to later decrease it
    }
}

fn log_slots(input: Res<ButtonInput<KeyCode>>, ui_inventory_slots: Query<(&UiInventorySlot, Entity), With<UiSlot>>) {
    if input.just_pressed(KeyCode::KeyP) {
        for slot in &ui_inventory_slots {
            println!("{:?}", slot.0);
        }
    }
}