use capture::{Capture, Rgb};
use threshold::Threshold;

mod capture;
mod threshold;

fn main() {
    let threshold = Threshold::builder()
        .target_min((49, 0, 129))
        .target_max_channel(90)
        .pointer_minimum_channel(248)
        .build();

    let capture = Capture::builder()
        .location((813, 165))
        .size((128, 128))
        .threshold(threshold)
        .build();

    let screenshot = capture.screenshot(Some(|(r, _, b), threshold| {
        let (min_r, _, min_b) = threshold.target_min();
        *r >= min_r && *r < threshold.target_max_channel() && *b >= min_b
    }));

    let one = vec![
        ((52, 48), (255, 255, 255)),
        ((51, 83), (255, 255, 255)),
        ((74, 74), (255, 255, 255)),
        ((71, 50), (255, 255, 255)),
        ((67, 83), (255, 255, 255)),
    ];

    let two = vec![
        ((76, 85), (255, 255, 255)),
        ((51, 83), (255, 255, 255)),
        ((74, 74), (255, 255, 255)),
        ((71, 50), (255, 255, 255)),
        ((67, 83), (255, 255, 255)),
    ];

    let three = vec![
        ((52, 48), (255, 255, 255)),
        ((51, 83), (255, 255, 255)),
        ((74, 74), (255, 255, 255)),
        ((71, 50), (255, 255, 255)),
        ((67, 83), (255, 255, 255)),
    ];

    let four = vec![
        ((52, 48), (255, 255, 255)),
        ((51, 83), (255, 255, 255)),
        ((74, 74), (255, 255, 255)),
        ((71, 50), (255, 255, 255)),
        ((67, 83), (255, 255, 255)),
    ];

    capture.on_change(&screenshot, |(r, _g, _b), image, threshold| {
        if *r >= threshold.pointer_minimum_channel() {
            let pixels = image
                .pixels()
                .take(128)
                .map(|pix| (pix.0[0], pix.0[1], pix.0[2]))
                .collect::<Vec<_>>();

            if is_number(&one, &pixels) {
                println!("one");
            } else if is_number(&two, &pixels) {
                println!("two");
            } else if is_number(&three, &pixels) {
                println!("three");
            } else if is_number(&four, &pixels) {
                println!("four");
            }
        }
    });
}

type Point = (u16, u16);
type Coordinate = (Point, Rgb);

fn is_number(coords: &Vec<Coordinate>, pixels: &Vec<Rgb>) -> bool {
    for ((x, y), coord_color) in coords.into_iter() {
        if let Some(pixel) = pixels.get((x * (y + 1)) as usize) {
            return *coord_color == *pixel;
        }
    }

    return false;
}
