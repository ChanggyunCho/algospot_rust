// https://algospot.com/judge/problem/read/MOON#

use std::io;

fn main() {
    let mut input = String::new();

    if let Err(error) = io::stdin().read_line(&mut input) {
        println!("Error: {}", error);
        return;
    }
    input.pop();
    let num_cases = input.parse::<i32>().unwrap();

    let mut result_vector: Vec<f64> = Vec::new();
    for _ in 0..num_cases {
        input.clear();
        if let Err(error) = io::stdin().read_line(&mut input) {
            println!("Error: {}", error);
            return;
        }
        input.pop();
        //println!("input: {}", input);

        let mut iter = input.split_whitespace();
        let mut m_radius = 0i32;
        let mut s_radius = 0i32;
        let mut distance = 0i32;
        if let Some(str) = iter.next() {
            m_radius = str.parse::<i32>().unwrap();
        }

        if let Some(str) = iter.next() {
            s_radius = str.parse::<i32>().unwrap();
        }

        if let Some(str) = iter.next() {
            distance = str.parse::<i32>().unwrap();
        }
        
        // Assume: 2 <= m_radius < s_radius < distance <= 12, distance < m_radius + s_radius
        let m_t = (m_radius.pow(2) - s_radius.pow(2) + distance.pow(2)) as f64 / (2 * distance) as f64;
        let height = (m_radius.pow(2) as f64 - m_t.powi(2)).sqrt();
        let m_size = std::f64::consts::PI * m_radius.pow(2) as f64;

        let moon_arc_size = m_radius.pow(2) as f64 * (height / m_radius as f64).asin();
        let moon_triangle_size = m_t * height;
        
        let shadow_arc_size = s_radius.pow(2) as f64 * (height / s_radius as f64).asin();
        let shadow_triangle_size = (distance as f64 - m_t) * height;

        let moon_circular_segment = moon_arc_size - moon_triangle_size;
        let shadow_circular_segment = shadow_arc_size - shadow_triangle_size;

        // push result to the vector
        result_vector.push(m_size - (moon_circular_segment + shadow_circular_segment));
    }

    // print result
    for value in result_vector {
        println!("{:.*}", 3, value);
    }
}
