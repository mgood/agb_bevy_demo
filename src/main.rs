#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

mod plugins;

use agb::display::object::Object;
use agb::fixnum::Vector2D;
use agb::include_aseprite;
use agb::input::Button;

include_aseprite!(
    mod sprites,
    "gfx/sprites.aseprite"
);

use agb::display::{Graphics, Rgb, Rgb15};
use bevy::app::PluginsState;
use bevy::input::gamepad::{
    GamepadConnection, GamepadConnectionEvent, RawGamepadButtonChangedEvent, RawGamepadEvent,
};
use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct ButtonController(agb::input::ButtonController);

/// Marker [`Component`] for the [`Entity`] that represents the gamepad built
/// into the GameBoy Advance.
#[derive(Component)]
#[non_exhaustive]
pub struct GameBoyGamepad {}

pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ButtonController(agb::input::ButtonController::new()))
            .add_systems(PreUpdate, update_buttons);
    }

    fn finish(&self, app: &mut App) {
        let world = app.world_mut();

        let gamepad = world.spawn(GameBoyGamepad {}).id();

        let event = GamepadConnectionEvent::new(
            gamepad,
            GamepadConnection::Connected {
                name: "GameBoy Advance Gamepad".to_string(),
                vendor_id: Some(0x057E),
                product_id: None,
            },
        );

        world.write_message::<RawGamepadEvent>(event.clone().into());
        world.write_message::<GamepadConnectionEvent>(event);
    }
}

fn update_buttons(
    mut btn: ResMut<ButtonController>,
    mut events: MessageWriter<RawGamepadEvent>,
    mut button_events: MessageWriter<RawGamepadButtonChangedEvent>,
    gamepad: Single<Entity, With<GameBoyGamepad>>,
) {
    btn.update();
    let gamepad = gamepad.into_inner();
    btn.is_just_pressed(Button::A)
        .then_some(1.)
        .or(btn.is_just_released(Button::A).then_some(0.))
        .map(|value| RawGamepadButtonChangedEvent::new(gamepad, GamepadButton::East, value))
        .map(|evt| {
            events.write(evt.into());
            button_events.write(evt);
        });
}

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // insert_non_send needs a 'static lifetime, so "leak" the gba struct which
    // will last for the life of the app regardless
    let gba = Box::leak(Box::new(gba));
    App::new()
        .add_plugins(ButtonPlugin)
        .add_plugins(bevy::input::InputPlugin)
        .insert_non_send(gba.graphics.get())
        .add_systems(Startup, (init_graphics, init_ball))
        .add_systems(Update, (move_ball, spawn_ball))
        .add_systems(Last, commit_frame)
        .set_runner(|mut app: App| -> AppExit {
            while app.plugins_state() == PluginsState::Adding {}

            app.finish();
            app.cleanup();

            loop {
                app.update();
                if let Some(exit) = app.should_exit() {
                    return exit;
                }
            }
        })
        .run();

    loop {}
}

fn init_graphics(mut gfx: NonSendMut<Graphics>) {
    gfx.set_background_palette_colour(0, 0, Rgb::new(0, 97, 132).into());
    gfx.set_background_palette_colour(0, 1, Rgb15::WHITE);
    gfx.set_background_palette_colour(0, 2, Rgb15::BLACK);
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Sprite {
    sprite: &'static agb::display::object::Sprite,
    pos: Vector2D<i32>,
}

fn init_ball(mut commands: Commands) {
    commands.spawn((
        Ball,
        Sprite {
            sprite: sprites::BALL.sprite(0),
            pos: (50, 50).into(),
        },
    ));
}

fn spawn_ball(commands: Commands, gamepad: Single<&Gamepad>) {
    if gamepad.just_pressed(GamepadButton::East) {
        init_ball(commands);
    }
}

fn move_ball(mut query: Query<&mut Sprite, With<Ball>>) {
    for mut sprite in &mut query {
        sprite.pos = sprite.pos + (1, 1).into();
    }
}

fn commit_frame(mut gfx: NonSendMut<Graphics>, sprites: Query<&Sprite>) {
    let mut frame = gfx.frame();
    for sprite in &sprites {
        Object::new(sprite.sprite)
            .set_pos(sprite.pos)
            .show(&mut frame);
    }
    frame.commit();
}
