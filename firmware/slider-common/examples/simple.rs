use slider_common::{Slider, SliderAction};

fn main() {
    let mut slider = Slider::new();
    slider.act(SliderAction::Goto(10));

    println!("Slider is at {}", slider.position());
    println!("Slider is at {:?}", slider.destination());

    slider.act(SliderAction::Goto(20));

    println!("Slider is at {}", slider.position());
}
