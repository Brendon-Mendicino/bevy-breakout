use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroupShaderType;

use crate::menu::components::column::{
    change_selected, change_selected_color, scroll_list, ScrollingList, SelectedItem,
};
use crate::menu::components::spacer::Spacer;
use crate::menu::components::text::UiText;
use crate::{ui_column_scrollable, AppState};

mod components;

#[derive(Resource)]
pub struct MenuData {
    button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_menu)
            .add_systems(
                Update,
                (
                    menu_key,
                    scroll_list,
                    change_selected,
                    change_selected_color,
                )
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu);
    }
}

fn setup_menu(mut commands: Commands) {
    let button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn(Spacer::around(Val::Px(100.)).style(|s| {
                    s.overflow = Overflow::clip_y();
                }))
                .with_children(|builder| {
                    ui_column_scrollable![
                        builder,
                        UiText::new("Start new game!"),
                        UiText::new("Settings."),
                    ];
                });
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}

fn menu_key(mut next_state: ResMut<NextState<AppState>>, input: Res<ButtonInput<KeyCode>>) {
    if input.pressed(KeyCode::Enter) {
        next_state.set(AppState::Game)
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}
