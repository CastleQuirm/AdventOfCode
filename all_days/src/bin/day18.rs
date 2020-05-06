// Thoughts
// Want to parse and analyse the map to create a hash-map:
// (source key, dest key) -> (distance, doors crossed)
// for all source key, dest key pairs (source keys have to include start, dest don't)

// Then we start building paths
// struct Path: (dest key, distance, [collected keys])
// We iteratively build vectors of possible paths of certain numbers of keys
// Initial path0vec is single element: (start, 0, [])

// Then we call a function on the vec, feeding the output in each time as a new
// input.
// Process: for each element in the vec, determine all next steps
// to get a next step: pick a key not in collected keys, check current dest key -> chosen key in hashmap
// if content of doors crossed for that element includes anything not in collected keys, drop it
// otherwise new path element: (chosen key, distance + path len, collected keys + chosen key)
// Once we've collected all the keys, look for the path with the min distance.


fn main() {
    println!("Part 1 Answer: {}", "unknown");
    println!("Part 2 Answer: {}", "unknown");
}
