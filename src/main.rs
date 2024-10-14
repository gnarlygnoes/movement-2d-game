pub mod game;

use game::camera::Camera;
// use crate::game::*;
use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;

// const WINDOW_SIZE: i32 = 1920;

fn init_window() -> Conf {
    Conf {
        window_title: String::from("Movement 2d Game"),
        window_resizable: true,
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}

#[macroquad::main(init_window)]
async fn main() {
    let tileset = load_texture("assets/sample_tile_set.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("assets/maze01.json").await.unwrap();
    let tiled_map =
        tiled::load_map(&tiled_map_json, &[("sample_tile_set.png", tileset)], &[]).unwrap();

    let mut static_colliders = vec![];
    for (_x, _y, tile) in tiled_map.tiles("sample_layer", None) {
        static_colliders.push(if tile.is_some() {
            Tile::Solid
        } else {
            Tile::Empty
        });
    }

    let tile_size = 108.;
    let w = screen_width();
    let h = screen_height();

    let mut world = World::new();
    world.add_static_tiled_layer(static_colliders, tile_size, tile_size, 15, 1);

    let p_size = 20;
    let mut player = game::mover::Mover {
        collider: world.add_actor(vec2(50.0, 80.0), p_size, p_size),
        speed: vec2(0., 0.),
        size: p_size as f32,
    };
    let mut cam = Camera {
        camera: Camera2D {
            zoom: Vec2 { x: 1., y: 1. },
            ..Default::default()
        },
        width: 500.,
        height: 500.,
    };
    loop {
        clear_background(GREEN);
        // set_camera(&init_camera(&Vec2 { x: 500., y: 500. }, w, h));
        set_camera(&cam.camera);

        tiled_map.draw_tiles(
            "sample_layer",
            Rect::new(0.0, 0.0, tile_size * 15., tile_size * 15.),
            None,
        );

        // Draw the player.
        {
            let pos = world.actor_pos(player.collider);
            draw_circle(
                pos.x + player.size / 2. - 1.,
                pos.y + player.size / 2. - 1.,
                player.size / 2.,
                BLUE,
            );
        }

        // player movement and collision
        {
            let pos = world.actor_pos(player.collider);
            let speed = 600.;

            if (is_key_down(KeyCode::D) || is_key_down(KeyCode::Right)) && pos.x + player.size < w {
                player.speed.x = speed
            } else if (is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)) && pos.x > 0. {
                player.speed.x = -speed
            } else {
                player.speed.x = 0.
            }

            if (is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)) && pos.y > 0. {
                player.speed.y = -speed
            } else if (is_key_down(KeyCode::S) || is_key_down(KeyCode::Down))
                && pos.y + player.size < h
            {
                player.speed.y = speed
            } else {
                player.speed.y = 0.
            }

            world.move_h(player.collider, player.speed.x * get_frame_time());
            world.move_v(player.collider, player.speed.y * get_frame_time());
        }

        // set_default_camera();

        let fps = get_fps();
        draw_text(format!("{fps}").as_str(), 30., 30., 36., RED);
        next_frame().await
    }
}
