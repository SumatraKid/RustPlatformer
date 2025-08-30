use std::thread;
use std::time::Duration;

use macroquad::{prelude::*};

mod entities {
    pub mod player;
    pub mod box_collider;
    pub mod coin;
}
use entities::player::*;
use entities::box_collider::*;
use entities::coin::*;

struct Object {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

// window settings
fn window_conf() -> Conf {
    Conf {
        window_title: "Platformer".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 650,
        window_resizable: false,
        ..Default::default()
    }
}

// collisions
fn collision_test<'a>(collider: &'a BoxCollider, tiles: &'a Vec<BoxCollider>,) -> Vec<&'a BoxCollider> {
    let mut collisions: Vec<&BoxCollider> = vec![];
    for tile in tiles {
        if collider.collided(&tile) {
            collisions.push(tile);
        }
    }
    return collisions;
}

#[macroquad::main(window_conf())]
async fn main() {

    // any code below will happen once the program starts ----------------------

        // tilemap -----------------------------------------------------------------
    let tilemap = 
        [['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', 'g', 'g', 'g', '0'],
        ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', 'g', '0', '0', '0', '0', '0', '0'],
        ['g', 'g', 'g', 'g', '0', '0', 'd', '0', '0', 'g', 'g', 'g', 'g'],
        ['d', 'd', 'd', 'd', 'g', 'g', 'd', 'g', 'g', 'd', 'd', 'd', 'd'],
        ['d', 'd', 'd', 'd', 'd', 'd', 'd', 'd', 'd', 'd', 'd', 'd', 'd'],
        ['d', 'd', 'd', 'd', 'd', 'd', 'd', 'd', 'd', 'd', 'd', 'd', 'd'],];

    let tile_size: f32 = 64.0;

    let mut tiles: Vec<BoxCollider> = vec![];
    
    for (index, row) in tilemap.iter().enumerate() {
        let y: f32 = index as f32;
        for (index, collumn) in row.iter().enumerate() {
            let x: f32 = index as f32;
            if collumn == &'g' || collumn == &'d' {
                let grass: BoxCollider = BoxCollider {
                    x: x * tile_size,
                    y: y * tile_size,
                    width: tile_size,
                    height: tile_size,
                };
                tiles.push(grass);
            }
        }
        
    }
        // ---------------------------------------------------------------------

        // player --------------------------------------------------------------
    let player_idle_texture: Texture2D = load_texture("assets/sprites/player/player_idle.png")
        .await
        .expect("FAILED TO LOAD THE PLAYER'S TEXTURE");
    player_idle_texture.set_filter(FilterMode::Nearest);

    let player_right_one_texture: Texture2D = load_texture("assets/sprites/player/player_right_one.png")
        .await
        .expect("FAILED TO LOAD THE PLAYER'S TEXTURE");
    player_right_one_texture.set_filter(FilterMode::Nearest);
    let player_left_one_texture: Texture2D = load_texture("assets/sprites/player/player_left_one.png")
        .await
        .expect("FAILED TO LOAD THE PLAYER'S TEXTURE");
    player_left_one_texture.set_filter(FilterMode::Nearest);

    let player_textures: [Texture2D; 3] = [player_idle_texture, player_right_one_texture, player_left_one_texture];

    let mut player: Player = Player {
        x: 100.0,
        y: 100.0,
        x_velocity: 0.0,
        y_velocity: 0.0,
        speed: 5.0,
        jump_height: 12.0,
        gravity: 0.5,
        on_ground: false,
        texture: player_textures,
        collider: BoxCollider { x: 100.0, y: 100.0, width: tile_size - 40.0, height: tile_size },
    };

        // ---------------------------------------------------------------------

        // coin ----------------------------------------------------------------

        let mut coin: Coin = Coin {
            x: 200.0,
            y: 300.0,
            width: 30.0,
            height: 30.0,
            value: 1,
            collider: BoxCollider {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            destroyed: false,
        };
        coin.collider.x = coin.x;
        coin.collider.y = coin.y;
        coin.collider.width = coin.width;
        coin.collider.height = coin.height;
        // ---------------------------------------------------------------------

        // score ---------------------------------------------------------------
    let score_object: Object = Object {
        x: 100.0,
        y: 100.0,
        width: 60.0,
        height: 0.0,
    };
    let mut score: i16 = 0;
        // ---------------------------------------------------------------------        

        // tile textures -------------------------------------------------------
    let tile_top_texture: Texture2D = load_texture("assets/tiles/top_tile.png")
        .await
        .expect("FAILED TO LOAD THE TILE_TOP_TEXTURE'S TEXTURE");
    tile_top_texture.set_filter(FilterMode::Nearest);

    let tile_dirt_texture: Texture2D = load_texture("assets/tiles/dirt_tile.png")
        .await
        .expect("FAILED TO LOAD THE TILE_DIRT_TEXTURE'S TEXTURE");
    tile_dirt_texture.set_filter(FilterMode::Nearest);

    let tile_textures: [Texture2D; 2] = [tile_top_texture, tile_dirt_texture];

        // ---------------------------------------------------------------------

        // animation variables -------------------------------------------------
    let mut timer: f32 = 0.0;
    let mut frame_index: usize = 0;
    let frame_duraration: f32 = 0.2;
        // ---------------------------------------------------------------------

    // -------------------------------------------------------------------------

    // game loop ---------------------------------------------------------------
    loop {
        
        update(&mut player, &mut coin, &tiles, &mut score,);

        draw(&coin, &tilemap, tile_size, score, &score_object, &tile_textures);

        if player.x_velocity > 0.0 {
            timer += get_frame_time();
            if timer > frame_duraration {
                timer = 0.0;
                if frame_index == 1 {
                    frame_index = 0;
                }
                else if frame_index == 0 {
                    frame_index = 1;
                }
            }
        }
        else if player.x_velocity < 0.0 {
            timer += get_frame_time();
            if timer > frame_duraration {
                timer = 0.0;
                if frame_index == 2 {
                    frame_index = 0;
                }
                else if frame_index == 0 {
                    frame_index = 2;
                }
            }
        }
        else {
            frame_index = 0;
            timer = frame_duraration;
        }
        

        draw_texture_ex(&player.render(frame_index), player.x - 20.0, player.y, WHITE, DrawTextureParams { 
            dest_size: Some(vec2(tile_size, tile_size)),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: Some(vec2(0.0, 0.0))
        });

        thread::sleep(Duration::from_millis(16));
        next_frame().await;
    }
    // -------------------------------------------------------------------------
}

// updates game logic once per frame
fn update(player: &mut Player, coin: &mut Coin, tiles: &Vec<BoxCollider>, score: &mut i16) {
    
    // player movement ---------------------------------------------------------
    player._platformer_movement();

    player.x += player.x_velocity;
    player.collider.x = player.x;

    // the player's y collision
    let collisions = collision_test(&player.collider, tiles);
    for tile in collisions {
        if player.x_velocity > 0.0 {
            player.x = tile.x - player.collider.width;
            player.x_velocity = 0.0;
        }
        if player.x_velocity < 0.0 {
            player.x = tile.x + tile.width;
            player.x_velocity = 0.0;
        }
    }
    player.collider.x = player.x;

    player.y += player.y_velocity;
    player.collider.y = player.y;

    // the player's y collision
    let vert_collisions = collision_test(&player.collider, tiles);
    for tile in vert_collisions {
        if player.y_velocity > 0.0 {
            player.y = tile.y - player.collider.height;
            player.y_velocity = 0.0;
            player.on_ground = true;
        }
        if player.y_velocity < 0.0 {
            player.y = tile.y + tile.height;
            player.y_velocity = 0.0;
        }
    }
    player.collider.y = player.y;
    if player.y_velocity != 0.0 {
        player.on_ground = false;
    }
    // -------------------------------------------------------------------------

    if coin.collider.collided(&player.collider) && coin.destroyed == false {
        coin.destroyed = true;
        *score = *score + coin.value;
    }

}

// draws once per frame
fn draw(coin: &Coin, tilemap: &[[char; 13]; 11], tile_size: f32, score: i16, score_object: &Object, tile_textures: &[Texture2D; 2]) {

    clear_background(Color { r: 0.388, g: 0.361, b: 0.427, a: 1.0});

    for (index, row) in tilemap.iter().enumerate() {
        let y: f32 = index as f32;
        for (index, collumn) in row.iter().enumerate() {
            let x: f32 = index as f32;
            if collumn == &'g' {
                draw_texture_ex(&tile_textures[0], x * tile_size, y * tile_size, WHITE, DrawTextureParams { 
                    dest_size: Some(vec2(tile_size, tile_size)),
                    source: None, rotation: 0.0,
                    flip_x: false, flip_y: false,
                    pivot: Some(vec2(0.0, 0.0))
                });
            }
            if collumn == &'d' {
                draw_texture_ex(&tile_textures[1], x * tile_size, y * tile_size, WHITE, DrawTextureParams { 
                    dest_size: Some(vec2(tile_size, tile_size)),
                    source: None, rotation: 0.0,
                    flip_x: false, flip_y: false,
                    pivot: Some(vec2(0.0, 0.0))
                });
            }
        }
        
    }

    if coin.destroyed == false {
        draw_rectangle(coin.x, coin.y, coin.width, coin.height, YELLOW);
        let _ = coin;
    }

    draw_text(&score.to_string(), score_object.x, score_object.y, score_object.width + score_object.height, BLACK);

    
}
