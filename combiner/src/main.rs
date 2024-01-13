mod args;
use args::Args;

fn main() {
    let args = Args::new();
    println!("Image 1: {}", args.image_1);
    println!("Image 2: {}", args.image_2);
    println!("Output: {}", args.output);
}
```