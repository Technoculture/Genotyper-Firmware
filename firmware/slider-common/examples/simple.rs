use slider_common::{Position, Range, Slider, SliderAction};

// /// Loop until the slider reaches the destination.
// /// Keep calling tick() until the slider has reached the destination.
fn execute_action(slider: &mut Slider) {
    while slider.position() != slider.destination().unwrap() {
        slider.tick();
        print!("{:?} ", slider.position().0);
    }

    println!("\n └──► Reached -> {:?}", slider.position().0);
}

fn main() {
    const RANGE: Range = Range { min: 0, max: 1000 };
    let mut slider = Slider::new(&RANGE);
    slider.act(SliderAction::Goto(Position(10, &RANGE)));

    println!(
        "Slider is at {:?}, Slider's destination is set to {:?}",
        slider.position(),
        slider.destination().unwrap()
    );

    let list_of_positions = vec![24, 7, 80, 30, 110, 0, 10];
    for pos in list_of_positions {
        slider.act(SliderAction::Goto(Position(pos, &RANGE)));
        execute_action(&mut slider);
    }
}
