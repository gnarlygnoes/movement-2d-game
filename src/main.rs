use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;

struct Player {
    collider: Actor,
    speed: Vec2,
}

fn InitPlayer(mut w: World) -> Player {
    return Player {
        collider: w.add_actor(vec2(50.0, 80.0), 8, 8),
        speed: vec2(0., 0.),
    };
}

#[macroquad::main("Movement 2d Game")]
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

    let mut world = World::new();
    world.add_static_tiled_layer(static_colliders, 8., 8., 40, 1);

    let mut player = Player {
        collider: world.add_actor(vec2(50.0, 80.0), 8, 8),
        speed: vec2(0., 0.),
    };
    // InitPlayer(world);

    let screen_size = (108. * 15.) * 0.7;
    request_new_screen_size(screen_size, screen_size);

    loop {
        clear_background(WHITE);

        tiled_map.draw_tiles(
            "sample_layer",
            Rect::new(0.0, 0.0, screen_size, screen_size),
            None,
        );
        {
            let pos = world.actor_pos(player.collider);
            // tiled_map.spr("tileset", 120, Rect::new(pos.x + 8.0, pos.y, -8., 8.))
            draw_circle(pos.x, pos.y, 20., RED);
        }

        next_frame().await
    }
}
