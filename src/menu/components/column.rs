use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

pub(super) const COLUMN_STYLE: Style = Style::DEFAULT;

#[derive(Component, Default, Debug)]
pub(crate) struct ScrollingList {
    pub(crate) position: f32,
}

#[derive(Component, Debug)]
pub(crate) struct SelectedItem {
    pub(crate) selected: bool,
    pub(crate) prev: Entity,
    pub(crate) next: Entity,
}

#[macro_export]
macro_rules! ui_column {
    ($parent:expr, $($x:expr),+ $(,)?) => (
        $parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            $(builder.spawn($x);)+
        })
    );
}

#[macro_export]
macro_rules! ui_column_scrollable {
    ($parent:expr, $($x:expr),+ $(,)?) => (
        $parent.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Interaction::default(),
            ScrollingList::default(),
            AccessibilityNode(NodeBuilder::new(Role::List)),
        ))
        .with_children(|builder| {
            let mut children = [$(builder.spawn((
                $x,
                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
            )).id()),+];

            for i in 0..children.len() {
                let sel = SelectedItem {
                    selected: i == 0,
                    next: children[(i + 1).rem_euclid(children.len())],
                    prev: children[(i as isize - 1).rem_euclid(children.len() as isize) as usize],
                };

                builder.add_command(move |c: &mut World| {
                    c.get_entity_mut(children[i]).unwrap().insert(sel);
                });
            }
        })
    );
}

pub(crate) fn scroll_list(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node, &Interaction)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node, interaction) in &mut query_list {
            // If list is not hovered go to the next one
            if !matches!(interaction, Interaction::Hovered) {
                continue;
            }

            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
            dbg!(max_scroll, style.top, dy);
        }
    }
}

pub(crate) fn change_selected(
    input: Res<ButtonInput<KeyCode>>,
    mut selected: Query<(&mut SelectedItem)>,
) {
    let go_down = if input.just_pressed(KeyCode::ArrowUp) {
        true
    } else if input.just_pressed(KeyCode::ArrowDown) {
        false
    } else {
        return;
    };

    let mut sel = selected.iter_mut().filter(|s| s.selected).next().unwrap();
    sel.selected = false;

    // Change the currently selected item
    let to_change = if go_down { sel.next } else { sel.prev };
    selected.get_mut(to_change).unwrap().selected = true;
}

pub(crate) fn change_selected_color(
    mut query: Query<(&mut BackgroundColor, &SelectedItem), Changed<SelectedItem>>,
) {
    for (mut background, selected) in &mut query {
        if selected.selected {
            background.0 = Color::RED;
        } else {
            background.0 = Color::default();
        }
    }
}
