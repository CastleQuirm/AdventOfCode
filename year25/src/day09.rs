// Potential improvements:
//

use std::str::FromStr;

use grid::coord::Coord2;
use itertools::Itertools;

pub fn day09(input_lines: &[Vec<String>]) -> (String, String) {
    let corners = input_lines[0]
        .iter()
        .map(|line| Coord2::from_str(line).expect("Bad coord"))
        .collect::<Vec<Coord2>>();
    // let collapsed_corners = collapse_corners(&corners);
    let mut edges = corners
        .windows(2)
        .map(|pair| Edge::from_coords(pair))
        .collect::<Vec<Edge>>();
    edges.push(Edge::from_coords(&[
        *corners.first().unwrap(),
        *corners.last().unwrap(),
    ]));
    let shape_v_edges = edges
        .iter()
        .filter_map(|e| match e {
            Edge::Vertical { line } => Some(line),
            Edge::Horizontal { .. } => None,
        })
        .cloned()
        .collect::<Vec<VerticalLine>>();
    let shape_h_edges = edges
        .iter()
        .filter_map(|e| match e {
            Edge::Vertical { .. } => None,
            Edge::Horizontal { line } => Some(line),
        })
        .cloned()
        .collect::<Vec<HorizontalLine>>();

    let (all_areas, red_green_areas): (Vec<i64>, Vec<Option<i64>>) = corners
        .into_iter()
        // .enumerate()
        .combinations(2)
        .map(|pair| {
            let delta = pair[0].diff(&pair[1]);
            let area = ((delta.x + 1) * (delta.y + 1)).abs();
            // let collapsed_pair = pair
            //     .iter()
            //     .map(|&(ix, _)| collapsed_corners[ix])
            //     .collect::<Vec<Coord2>>();
            if in_bounds(pair, &shape_h_edges, &shape_v_edges) {
                (area, Some(area))
            } else {
                (area, None)
            }
        })
        .unzip();

    let answer1 = all_areas.iter().max().unwrap();
    let answer2 = red_green_areas.iter().flatten().max().unwrap();

    (format!("{}", answer1), format!("{}", answer2))
}

fn in_bounds(pair: Vec<Coord2>, h_edges: &[HorizontalLine], v_edges: &[VerticalLine]) -> bool {
    // The pair defines a rectangle.
    let left = pair[0].x.min(pair[1].x);
    let right = pair[0].x.max(pair[1].x);
    let bottom = pair[0].y.min(pair[1].y);
    let top = pair[0].y.max(pair[1].y);
    let bottom_border = HorizontalLine {
        left,
        right,
        y: bottom,
    };
    let top_border = HorizontalLine {
        left,
        right,
        y: top,
    };
    let left_border = VerticalLine {
        low: bottom,
        high: top,
        x: left,
    };
    let right_border = VerticalLine {
        low: bottom,
        high: top,
        x: right,
    };

    // Check that the lines they create don't crossover the full shape edges.
    let enable_trace = (pair[0] == Coord2 { x: 9, y: 5 } && pair[1] == Coord2 { x: 2, y: 3 })
        || (pair[1] == Coord2 { x: 9, y: 5 } && pair[0] == Coord2 { x: 2, y: 3 });

    if h_edges
        .iter()
        .any(|edge| crossover(edge, &left_border) || crossover(edge, &right_border))
        || v_edges
            .iter()
            .any(|edge| crossover(&bottom_border, edge) || crossover(&top_border, edge))
    {
        return false;
    }

    // Need to check the edges fall into the shape.
    // let test_square = vec![
    //     Edge::Vertical { line: left_border },
    //     Edge::Vertical { line: right_border },
    //     Edge::Horizontal {
    //         line: bottom_border,
    //     },
    //     Edge::Horizontal { line: top_border },
    // ];

    // // test_square.iter().all(|edge| )

    // let left_check = left_border.contained_in_shape(v_edges);
    // let right_check = right_border.contained_in_shape(v_edges);
    // let top_check = top_border.contained_in_shape(h_edges);
    // let bottom_check = bo

    left_border.contained_in_shape(v_edges, enable_trace)
        && right_border.contained_in_shape(v_edges, enable_trace)
        && top_border.contained_in_shape(h_edges, enable_trace)
        && bottom_border.contained_in_shape(h_edges, enable_trace)
}

// // Convert the diagram into a smaller version.  Each entry maps to the one in the same position
// // but with a distance of exactly 2 between every consecutive pair of values (per axis).
// fn collapse_corners(corners: &[Coord2]) -> Vec<Coord2> {
//     let (mut x_values, mut y_values): (Vec<i64>, Vec<i64>) =
//         corners.iter().map(|c| (c.x, c.y)).unzip();
//     x_values.sort();
//     y_values.sort();
//     let x_map = x_values
//         .iter()
//         .enumerate()
//         .map(|(i, p)| (*p, i * 2))
//         .collect::<HashMap<i64, usize>>();
//     let y_map = y_values
//         .iter()
//         .enumerate()
//         .map(|(i, p)| (*p, i * 2))
//         .collect::<HashMap<i64, usize>>();
//     corners
//         .iter()
//         .map(|c| {
//             Coord2::from((
//                 *x_map.get(&c.x).expect("Unrecognised x") as i64,
//                 *y_map.get(&c.y).expect("Unrecognised y") as i64,
//             ))
//         })
//         .collect::<Vec<Coord2>>()
// }

#[derive(Clone)]
enum Edge {
    Vertical { line: VerticalLine },
    Horizontal { line: HorizontalLine },
}

impl Edge {
    // fn cross(&self, other: &Self) -> bool {
    //     match (self, other) {
    //         (Edge::Vertical { line: v }, Edge::Horizontal { line: h }) => crossover(h, v),
    //         (Edge::Horizontal { line: h }, Edge::Vertical { line: v }) => crossover(h, v),
    //         _ => false,
    //     }
    // }

    // fn contained_in_shape(&self, shape_h_edges: &[HorizontalLine], shape_v_edges: &[VerticalLine]) -> bool {
    //     match self {
    //         Edge::Vertical { line } => line.contained_in_shape(shape_v_edges),
    //         Edge::Horizontal { line } => line.contained_in_shape(shape_h_edges),
    //     }
    // }

    fn from_coords(coords: &[Coord2]) -> Self {
        assert_eq!(coords.len(), 2);
        assert!((coords[0].x == coords[1].x) || (coords[0].y == coords[1].y));
        if coords[0].x == coords[1].x {
            Edge::Vertical {
                line: VerticalLine {
                    low: coords[0].y.min(coords[1].y),
                    high: coords[0].y.max(coords[1].y),
                    x: coords[0].x,
                },
            }
        } else {
            assert_eq!(coords[0].y, coords[1].y);
            Edge::Horizontal {
                line: HorizontalLine {
                    left: coords[0].x.min(coords[1].x),
                    right: coords[0].x.max(coords[1].x),
                    y: coords[0].y,
                },
            }
        }
    }
}

#[derive(Clone)]
struct VerticalLine {
    low: i64,
    high: i64,
    x: i64,
}

impl VerticalLine {
    fn includes_point(&self, point: &Coord2) -> bool {
        point.x == self.x && point.y >= self.low && point.y <= self.high
    }

    fn contained_in_shape(&self, v_edges: &[VerticalLine], enable_trace: bool) -> bool {
        let result = (self.low..=self.high).all(|check_y| {
            v_edges.iter().any(|edge| {
                println!("- Check y: {check_y}");
                edge.includes_point(&Coord2 {
                    x: self.x,
                    y: check_y,
                })
            }) || v_edges
                .iter()
                .filter(|edge| edge.low <= check_y && edge.high >= check_y && edge.x < self.x)
                .count()
                % 2
                == 1
        });
        if enable_trace {
            println!("Contained result: {result}");
        }
        result
    }
}

#[derive(Clone)]
struct HorizontalLine {
    left: i64,
    right: i64,
    y: i64,
}

impl HorizontalLine {
    fn includes_point(&self, point: &Coord2) -> bool {
        point.y == self.y && point.x >= self.left && point.x <= self.right
    }

    fn contained_in_shape(&self, h_edges: &[HorizontalLine], enable_trace: bool) -> bool {
        let result = (self.left..=self.right).all(|check_x| {
            h_edges.iter().any(|edge| {
                edge.includes_point(&Coord2 {
                    x: check_x,
                    y: self.y,
                })
            }) || h_edges
                .iter()
                .filter(|edge| edge.left <= check_x && edge.right >= check_x && edge.y < self.y)
                .count()
                % 2
                == 1
        });
        if enable_trace {
            println!("Contained result: {result}");
        }
        result
    }
}

fn crossover(h: &HorizontalLine, v: &VerticalLine) -> bool {
    h.y > v.low && h.y < v.high && v.x > h.left && v.x < h.right
}

#[cfg(test)]
mod tests {
    use super::day09;
    use crate::utils::load_input;

    #[test]
    fn check_day09_case01() {
        full_test(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3", // INPUT STRING
            "50", // PART 1 RESULT
            "24", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day09(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
