use slider_common::{Slider, SliderAction};

/// Loop until the slider reaches the destination.
/// Keep calling tick() until the slider has reached the destination.
fn execute_action(slider: &mut Slider) {
    while slider.position() != slider.destination().unwrap() {
        slider.tick();
        println!("Slider is at {}", slider.position());
    }

    println!("             └──► Reached.");
}

fn main() {
    let mut slider = Slider::new();
    slider.act(SliderAction::Goto(10));

    println!(
        "Slider is at {}, Slider's destination is set to {:?}",
        slider.position(),
        slider.destination().unwrap()
    );

    slider.act(SliderAction::Goto(20));
    execute_action(&mut slider);
    slider.act(SliderAction::Goto(5));
    execute_action(&mut slider)
}
