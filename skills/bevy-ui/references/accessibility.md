# Bevy 0.19 UI — Accessibility (`InputFocus`)

## Quick reference

| Item | Purpose |
|---|---|
| `InputFocus` | Resource tracking which entity currently has logical focus. |
| `bevy::input_focus::InputFocus` | Full import path. |
| `InputFocusPlugin` | Included by `DefaultPlugins`; initializes focus state. |
| `input_focus.set(entity, cause)` | Give focus and record why it changed. |
| `input_focus.clear()` | Clear focus (e.g. on `Interaction::None`). |
| `input_focus.get()` | Returns `Option<Entity>` — the currently focused entity. |

## Why `InputFocus` exists

Bevy integrates with platform accessibility trees (AT — screen readers, switch
access, magnification software) via `bevy_a11y`. The `InputFocus` resource is the
bridge: when you call `input_focus.set(entity, cause)`, Bevy notifies the AT that focus
has moved to that entity. Without this, keyboard-only or screen-reader users
receive no feedback when a button is hovered or pressed.

`InputFocus` also drives Bevy's internal focus-dispatch system: `KeyboardInput`
events are routed to the focused entity rather than broadcast globally.

## Common patterns

### Minimal setup

```rust
use bevy::{input_focus::InputFocus, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}
```

### Setting and clearing focus in an interaction system

```rust
use bevy::{input_focus::{FocusCause, InputFocus}, prelude::*};

fn button_system(
    mut input_focus: ResMut<InputFocus>,
    mut query: Query<(Entity, &Interaction, &mut Button), Changed<Interaction>>,
) {
    for (entity, interaction, mut button) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity, FocusCause::Pressed);
                button.set_changed(); // notify a11y system
            }
            Interaction::Hovered => {
                input_focus.set(entity, FocusCause::Navigated);
                button.set_changed(); // notify a11y system
            }
            Interaction::None => {
                input_focus.clear();
            }
        }
    }
}
```

### Reading the current focus (e.g. for custom keyboard handling)

```rust
use bevy::{input_focus::InputFocus, prelude::*};

fn keyboard_dispatch(
    input_focus: Res<InputFocus>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        if let Some(focused) = input_focus.get() {
            println!("Enter pressed while {focused:?} is focused");
        }
    }
}
```

## Pitfalls

- **`DefaultPlugins` initializes `InputFocus`.** Explicit initialization is only
  needed when a custom plugin set omits `InputFocusPlugin`.

- **`input_focus` path changed in 0.18.** The correct import is
  `bevy::input_focus::InputFocus`. It is *not* re-exported from `bevy::prelude`
  in 0.18 — you must import it explicitly.

- **`button.set_changed()` and `InputFocus` work together.** Calling only one
  without the other may result in the AT being notified but not updating, or
  updating without the button's accessibility node being refreshed. Always call
  both in `Pressed` and `Hovered` arms.

- **Clearing focus at `Interaction::None` is intentional.** If you do not clear
  focus when the cursor leaves a button, the AT will still consider that button
  focused even when no button is visually highlighted. This is confusing to AT users.
