use image::GenericImageView;

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("map1.bmp").unwrap();

    // The dimensions method returns the images width and height.
    let dimensions = img.dimensions();
    println!("dimensions {:?}", dimensions);

    let cap = dimensions.0 as usize * dimensions.1 as usize;
    let mut map_blocks = Vec::with_capacity(cap);
    map_blocks.resize_with(cap, || 0);

    // The color method returns the image's `ColorType`.
    // println!("{:?}", img.color());

    println!("// generated:\n\n");
    println!("let map: [[u8;{}];{}] = [", dimensions.0, dimensions.1);
    for (index, (x,y,pixel)) in img.pixels().enumerate() {
        if x == 0 {
            print!("    [");
        }
        let new_block = if pixel.0[0] > 0 { 1u8 } else { 0u8 };
        map_blocks[index] = new_block;
        print!("{new_block}");
        if x == dimensions.0-1 {
            println!("],");
        } else {
            print!(",");
        }
    }
    println!("];");

    let arr = [
        [0,0,0,0,0,0],
        [0,0,0,0,0,0],
    ];

    // Write the contents of this image to the Writer in PNG format.
    img.save("test.png").unwrap();

    std::fs::write("./map.bin", map_blocks).expect("writing map bytes");
}