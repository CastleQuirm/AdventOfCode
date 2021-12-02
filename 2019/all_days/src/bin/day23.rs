use all_days::Computer;

fn main() {
    let orig_computer = all_days::define_computer("input/day23.txt");
    let mut computer_vec: Vec<Computer> = Vec::new();

    for i in 0..50 {
        let mut computer = orig_computer.clone_computer();
        computer.push_to_input(&vec![i]);
        computer_vec.push(computer);
    }

    let mut rcvd_255: bool = false;
    let mut next_computer = 0;
    let mut computers_since_packet_sent = 0;
    let (mut nat_x, mut nat_y) = (0, 0);
    let mut delivered_y = 0;

    'outer: loop {
        let output = computer_vec[next_computer].single_force_run_computer();
        if output.len() % 3 != 0 {
            panic!("Expected triplets of output, got len {}: {:?}", output.len(), output);
        }
        for i in 0..output.len() / 3 {
            if output[3 * i] == 255 {
                if !rcvd_255 {
                    println!("Part 1 Answer: {}", output[3 * i + 2]);
                }
                rcvd_255 = true;
                nat_x = output[3 * i + 1];
                nat_y = output[3 * i + 2];
                continue;
            }
            computer_vec[output[3 * i] as usize].push_to_input(&vec![output[3 * i + 1], output[3 * i + 2]]);
            computers_since_packet_sent = 0;
        }
        if next_computer == 49 {
            next_computer = 0;
        } else {
            next_computer += 1;
        }
        computers_since_packet_sent += 1;
        if computers_since_packet_sent > 1000 {
            if nat_y == delivered_y {
                println!("Part 2 Answer: {}", nat_y);
                break 'outer;
            }
            delivered_y = nat_y;
            computer_vec[0].push_to_input(&vec![nat_x, nat_y]);
            nat_x = 0;
            nat_y = 0;
            computers_since_packet_sent = 0;
        }
    }
}
