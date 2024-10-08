use notan::draw::*;
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    std::env::set_var("WINIT_X11_SCALE_FACTOR", "1.0");
    notan::init().add_config(DrawConfig).draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.rect((100.0, 100.0), (600.0, 400.0))
        // .top_left_radius(f32::EPSILON)
        .top_right_radius(90.0)
        .bottom_right_radius(90.0);
    gfx.render(&draw);
}
