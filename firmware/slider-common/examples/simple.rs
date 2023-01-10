use slider_common::{Position, Range, Slider, SliderAction};

/// Loop until the slider reaches the destination.
/// Keep calling tick() until the slider has reached the destination.
fn execute_action(slider: &mut Slider) {
    while slider.position() != slider.destination().unwrap() {
        slider.tick();
        print!("{:?} ", slider.position());
    }

    println!("\n └──► Reached -> {:?}", slider.position());
}

fn main() {
    const RANGE: Range = Range { min: 30, max: 100 };
    let mut slider = Slider::new(&RANGE);
    slider.act(SliderAction::Goto(Position::new(1000, &RANGE)));

    println!(
        "Slider is at {:?}, Slider's destination is set to {:?}",
        slider.position(),
        slider.destination()
    );

    let list_of_positions = vec![24, 7, 80, 30, 110, 0, 10];
    for pos in list_of_positions {
        slider.act(SliderAction::Goto(Position::new(pos, &RANGE)));
        execute_action(&mut slider);
    }
}
