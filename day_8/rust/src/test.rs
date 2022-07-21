use super::*;


#[test]
fn get_fewest_digit_layer_gives_2() {
    let s: String = String::from("000111222000111222000101222");

    let i = Image::new(3,3, s.chars().collect());

    let layer = i.get_fewest_digit_layer(1);

    assert_eq!(layer, 2);
}

#[test]
fn multiply_counts_gives_6() {
    let s: String = String::from("000111222000111222000101222");

    let i = Image::new(3,3, s.chars().collect());

    let layer = i.get_fewest_digit_layer(1);

    assert_eq!(layer, 2);

    let count = i.multiply_counts(layer, 1, 2);
    assert_eq!(count, 6);
}

#[test]
fn decode_image_gives_checker() {
    let s = String::from("0222112222120000");
    let i = Image::new(2,2, s.chars().collect());

    let image = i.decode_image();

    assert_eq!(image, vec![vec![0,1], vec![1,0]]);
}
