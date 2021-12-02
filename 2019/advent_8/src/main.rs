use std::fs;

// Might be good to work out how to use "Chunks" for separating the data

fn main() {
    let string = fs::read_to_string("input_image.txt")
        .expect("Failed to read file");

    let image_iter = string.trim().chars().map(|i| {
        i.to_string().parse().expect("")
    });

    let image: Vec<i32> = image_iter.collect();

    if image.len() % 150 != 0 { panic!("Not full layers"); }
    let mut layer_data: Vec<Vec<i32>> = Vec::new();

    for layer_number in 0..image.len()/150 {
        let start_index = layer_number * 150;
        layer_data.push(image[start_index..start_index+150].to_vec());
    }

    let mut min_zeroes = 151;
    let mut result_product = 0;

    for layer in &layer_data {
        // println!("layer {:?}", layer);
        let zeroes = count(&layer, 0);
        // println!("Found {} zeroes", zeroes);
        if zeroes < min_zeroes {
            min_zeroes = zeroes;
            result_product = count(&layer, 1) * count(&layer, 2);
        }
    }

    println!("Part 1 result: {}", result_product);

    let mut final_image: Vec<i32> = Vec::new();
    for i in 0..150 {
        for layer in &layer_data {
            if layer[i] != 2 {
                final_image.push(layer[i]);
                break;
            }
        }
    }

    let mut image_rows: Vec<Vec<i32>> = Vec::new();

    for row_number in 0..6 {
        let start_index = row_number * 25;
        image_rows.push(final_image[start_index..start_index+25].to_vec());
        println!("{:?}", image_rows[row_number]);
    }

    if count(&final_image, 2) != 0 { panic!("Some still transparent!"); }
    // println!("Final image {:?}", image_rows);
}

fn count(layer: &Vec<i32>, search_value: i32) -> i32 {
    (layer.split(|num| *num == search_value).count() - 1) as i32
}
