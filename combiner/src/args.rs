fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

pub struct Args {
    image_1: String,
    image_2: String,
    output: String,
}