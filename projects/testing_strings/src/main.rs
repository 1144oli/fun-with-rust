use boxy_cli::prelude::*;

fn main() {
    Boxy::builder()
        .box_type(BoxType::Double)
        .color("#00ffff")
        .add_segment("Hello, Boxy!", "#ffffff", BoxAlign::Center)
        .add_segment("A beautiful CLI box library", "#32CD32", BoxAlign::Center)
        .add_segment("Hello","#ffffff",BoxAlign::Right)
        .padding(BoxPad::uniform(1), BoxPad::vh(1, 2))
        .build()
        .display();
}