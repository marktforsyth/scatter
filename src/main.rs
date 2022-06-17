use nannou::prelude::*;
use nannou::state::mouse::ButtonPosition::{Up, Down};

#[derive(PartialEq)]
enum Mode {
    Pull,
    Push,
    // Ripple,
}

#[derive(PartialEq)]
enum GridColor {
    White,
    Cyan,
    HotPink,
    SpringGreen,
}

fn generate_grid(size: f32) -> [[[f32; 2]; 280]; 2] {
    let mut base_grid_coordinates: Vec<[f32; 2]> = vec![];
    let mut grid_coordinates: Vec<[f32; 2]> = vec![];

    for x in 0..20 {
        for y in 0..14 {
            base_grid_coordinates.push([x as f32 * size - size * 9.5, y as f32 * size - size * 6.5]);
            grid_coordinates.push([x as f32 * size - size * 9.5, y as f32 * size - size * 6.5]);
        }
    }

    [
        base_grid_coordinates.try_into().expect("Grid vector has the wrong length"),
        grid_coordinates.try_into().expect("Grid vector has the wrong length")
    ]
}

struct Model {
    base_grid: [[f32; 2]; 280],
    grid: [[f32; 2]; 280],
    mode: Mode,
    grid_color: GridColor,
    strength: u8,
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(app: &App) -> Model {
    app.main_window().set_maximized(true);

    let screen_height = app.main_window().inner_size_points().1 as f32;
    let grid_coordinates = generate_grid((screen_height - 130.0) / 13.0);

    Model {
        base_grid: grid_coordinates[0],
        grid: grid_coordinates[1],
        mode: Mode::Pull,
        grid_color: GridColor::White,
        strength: 50,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for (d, dot) in model.grid.iter_mut().enumerate() {
        let orig_dot = model.base_grid[d];
        let distance = (
            f32::powf(orig_dot[0] - app.mouse.x, 2.0) + f32::powf(orig_dot[1] - app.mouse.y, 2.0)
        ).sqrt();

        match model.mode {
            Mode::Pull => {
                if distance > model.strength.into() {
                    dot[0] = orig_dot[0] + (app.mouse.x - orig_dot[0]) / distance * model.strength as f32;
                    dot[1] = orig_dot[1] + (app.mouse.y - orig_dot[1]) / distance * model.strength as f32;
                } else {
                    dot[0] = app.mouse.x;
                    dot[1] = app.mouse.y;
                }
            },
            Mode::Push => {
                dot[0] = orig_dot[0] - (app.mouse.x - orig_dot[0]) / distance * model.strength as f32;
                dot[1] = orig_dot[1] - (app.mouse.y - orig_dot[1]) / distance * model.strength as f32;
            },
        }
    }

    match app.mouse.buttons.left() {
        Up => {},
        Down(pos) => {
            let screen_width = app.main_window().inner_size_points().0 as f32;
            let screen_height = app.main_window().inner_size_points().1 as f32;

            if pos[0] >= -screen_width / 2.0 + 20.0 && pos[0] <= -screen_width / 2.0 + 80.0 {
                if pos[1] <= screen_height / 2.0 - 20.0 && pos[1] >= screen_height / 2.0 - 80.0 {
                    if model.mode != Mode::Pull {
                        model.mode = Mode::Pull;

                        let grid_coordinates = generate_grid((screen_height - 130.0) / 13.0);
                        model.base_grid = grid_coordinates[0];
                        model.grid = grid_coordinates[1];
                    }
                } else if pos[1] <= screen_height / 2.0 - 100.0 && pos[1] >= screen_height / 2.0 - 160.0 {
                    if model.mode != Mode::Push {
                        model.mode = Mode::Push;

                        let grid_coordinates = generate_grid((screen_height - 150.0) / 14.0);
                        model.base_grid = grid_coordinates[0];
                        model.grid = grid_coordinates[1];
                    }
                } else if pos[1] <= screen_height / 2.0 - 260.0 && pos[1] >= screen_height / 2.0 - 320.0 {
                    if model.grid_color != GridColor::White {
                        model.grid_color = GridColor::White;
                    }
                } else if pos[1] <= screen_height / 2.0 - 340.0 && pos[1] >= screen_height / 2.0 - 400.0 {
                    if model.grid_color != GridColor::SpringGreen {
                        model.grid_color = GridColor::SpringGreen;
                    }
                } else if pos[1] <= screen_height / 2.0 - 420.0 && pos[1] >= screen_height / 2.0 - 480.0 {
                    if model.grid_color != GridColor::Cyan {
                        model.grid_color = GridColor::Cyan;
                    }
                } else if pos[1] <= screen_height / 2.0 - 500.0 && pos[1] >= screen_height / 2.0 - 560.0 {
                    if model.grid_color != GridColor::HotPink {
                        model.grid_color = GridColor::HotPink;
                    }
                }
            }
        },
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for dot in model.grid {
        let dot_color = match model.grid_color {
            GridColor::White => WHITE,
            GridColor::SpringGreen => SPRINGGREEN,
            GridColor::Cyan => CYAN,
            GridColor::HotPink => HOTPINK,
        };

        draw.ellipse().x_y(dot[0], dot[1]).radius(2.0).color(dot_color);
    }

    let screen_width = app.main_window().inner_size_points().0 as f32;
    let screen_height = app.main_window().inner_size_points().1 as f32;

    let mut btn_stroke_weights = [3.0, 2.0, 3.0, 2.0, 2.0, 2.0];
    let mut btn_stroke_colors = [WHITE, GRAY, WHITE, GRAY, GRAY, GRAY, GRAY];

    match model.mode {
        Mode::Pull => {
            btn_stroke_weights[0] = 3.0;
            btn_stroke_weights[1] = 2.0;

            btn_stroke_colors[0] = WHITE;
            btn_stroke_colors[1] = GRAY;
        },
        Mode::Push => {
            btn_stroke_weights[0] = 2.0;
            btn_stroke_weights[1] = 3.0;

            btn_stroke_colors[0] = GRAY;
            btn_stroke_colors[1] = WHITE;
        },
    }

    match model.grid_color {
        GridColor::White => {
            btn_stroke_weights[2] = 3.0;
            btn_stroke_weights[3] = 2.0;
            btn_stroke_weights[4] = 2.0;
            btn_stroke_weights[5] = 2.0;

            btn_stroke_colors[2] = WHITE;
            btn_stroke_colors[3] = GRAY;
            btn_stroke_colors[4] = GRAY;
            btn_stroke_colors[5] = GRAY;
        },
        GridColor::SpringGreen => {
            btn_stroke_weights[2] = 2.0;
            btn_stroke_weights[3] = 3.0;
            btn_stroke_weights[4] = 2.0;
            btn_stroke_weights[5] = 2.0;

            btn_stroke_colors[2] = GRAY;
            btn_stroke_colors[3] = WHITE;
            btn_stroke_colors[4] = GRAY;
            btn_stroke_colors[5] = GRAY;
        },
        GridColor::Cyan => {
            btn_stroke_weights[2] = 2.0;
            btn_stroke_weights[3] = 2.0;
            btn_stroke_weights[4] = 3.0;
            btn_stroke_weights[5] = 2.0;

            btn_stroke_colors[2] = GRAY;
            btn_stroke_colors[3] = GRAY;
            btn_stroke_colors[4] = WHITE;
            btn_stroke_colors[5] = GRAY;
        },
        GridColor::HotPink => {
            btn_stroke_weights[2] = 2.0;
            btn_stroke_weights[3] = 2.0;
            btn_stroke_weights[4] = 2.0;
            btn_stroke_weights[5] = 3.0;

            btn_stroke_colors[2] = GRAY;
            btn_stroke_colors[3] = GRAY;
            btn_stroke_colors[4] = GRAY;
            btn_stroke_colors[5] = WHITE;
        },
    }

    // draw.text()
    //     .new("MODE")
    //     .x_y(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 20.0);

    draw.ellipse()
        .x_y(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 50.0)
        .radius(30.0)
        .color(BLACK)
        .stroke(btn_stroke_colors[0])
        .stroke_weight(btn_stroke_weights[0]);
    draw.arrow()
        .start(pt2(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 35.0))
        .end(pt2(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 45.0))
        .weight(2.0)
        .head_length(4.0);
    draw.arrow()
        .start(pt2(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 65.0))
        .end(pt2(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 55.0))
        .weight(2.0)
        .head_length(4.0);
    draw.arrow()
        .start(pt2(-screen_width / 2.0 + 35.0, screen_height / 2.0 - 50.0))
        .end(pt2(-screen_width / 2.0 + 45.0, screen_height / 2.0 - 50.0))
        .weight(2.0)
        .head_length(4.0);
    draw.arrow()
        .start(pt2(-screen_width / 2.0 + 65.0, screen_height / 2.0 - 50.0))
        .end(pt2(-screen_width / 2.0 + 55.0, screen_height / 2.0 - 50.0))
        .weight(2.0)
        .head_length(4.0);


    draw.ellipse()
        .x_y(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 130.0)
        .radius(30.0)
        .color(BLACK)
        .stroke(btn_stroke_colors[1])
        .stroke_weight(btn_stroke_weights[1]);
    draw.arrow()
        .start(pt2(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 125.0))
        .end(pt2(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 115.0))
        .weight(2.0)
        .head_length(4.0);
    draw.arrow()
        .start(pt2(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 135.0))
        .end(pt2(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 145.0))
        .weight(2.0)
        .head_length(4.0);
    draw.arrow()
        .start(pt2(-screen_width / 2.0 + 45.0, screen_height / 2.0 - 130.0))
        .end(pt2(-screen_width / 2.0 + 35.0, screen_height / 2.0 - 130.0))
        .weight(2.0)
        .head_length(4.0);
    draw.arrow()
        .start(pt2(-screen_width / 2.0 + 55.0, screen_height / 2.0 - 130.0))
        .end(pt2(-screen_width / 2.0 + 65.0, screen_height / 2.0 - 130.0))
        .weight(2.0)
        .head_length(4.0);

    draw.ellipse()
        .x_y(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 290.0)
        .radius(30.0)
        .color(BLACK)
        .stroke(btn_stroke_colors[2])
        .stroke_weight(btn_stroke_weights[2]);
    draw.ellipse()
        .x_y(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 290.0)
        .radius(20.0)
        .color(WHITE);

    draw.ellipse()
        .x_y(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 370.0)
        .radius(30.0)
        .color(BLACK)
        .stroke(btn_stroke_colors[3])
        .stroke_weight(btn_stroke_weights[3]);
    draw.ellipse()
        .x_y(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 370.0)
        .radius(20.0)
        .color(SPRINGGREEN);

    draw.ellipse()
        .x_y(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 450.0)
        .radius(30.0)
        .color(BLACK)
        .stroke(btn_stroke_colors[4])
        .stroke_weight(btn_stroke_weights[4]);
    draw.ellipse()
        .x_y(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 450.0)
        .radius(20.0)
        .color(CYAN);

    draw.ellipse()
        .x_y(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 530.0)
        .radius(30.0)
        .color(BLACK)
        .stroke(btn_stroke_colors[5])
        .stroke_weight(btn_stroke_weights[5]);
    draw.ellipse()
        .x_y(-screen_width / 2.0 + 50.0, screen_height / 2.0 - 530.0)
        .radius(20.0)
        .color(HOTPINK);

    draw.to_frame(app, &frame).unwrap();
}

// fn resized(_app: &App, model: &mut Model, dimensions: Vec2) {
//     match model.mode {
//         Mode::Pull => {
//             let grid_coordinates = generate_grid((screen_height - 130.0) / 13.0);
//             model.base_grid = grid_coordinates[0];
//             model.grid = grid_coordinates[1];
//         },
//         Mode::Push => {
//             let grid_coordinates = generate_grid((screen_height - 150.0) / 14.0);
//             model.base_grid = grid_coordinates[0];
//             model.grid = grid_coordinates[1];
//         },
//     }
// }
