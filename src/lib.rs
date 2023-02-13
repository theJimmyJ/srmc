//! # SRMC (Small Rust Math Crate)
//!
//! `srmc` is a collection of mathematical methods, functions, and other useful tools. The goal is to make a stand alone math focused crate that can be used for a wide variety of applications.
//! Will be continuously updated and improved as I imporve with the Rust langauge.
//!
/// Creates a vector in numerical order from start to end with steps + 1 elements.
/// NOTE: Elements of vector should be evenly spaced, but often times last two are not. Working on a fix for it, but if evenly spaced values is vital you may need to remove the final value or choose a different step size.
///
/// # Examples
///
/// ```
/// let vec1: Vec<f32> = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
/// let vec2 = srmc::create_sequential_vec(0.0, 5.0, 5);
///
/// assert_eq!(vec1, vec2);
/// ```
/// ```
/// let vec3: Vec<f32> = vec![0.0, 1.5, 3.0, 4.5, 6.0, 7.5, 9.0, 10.5, 12.0, 13.5, 15.0];
/// let vec4 = srmc::create_sequential_vec(0.0, 15.0, 10);
///
/// assert_eq!(vec3, vec4);
/// ```

pub fn create_sequential_vec(start: f32, end: f32, steps: i32) -> Vec<f32> {
    let mut sequential_vec = Vec::new();
    let interval = (end - start) / steps as f32;
    let mut current_num = start;
    while current_num < end {
        sequential_vec.push(current_num);
        current_num = current_num + interval;
    }
    sequential_vec.push(end);
    return sequential_vec;
}

/// Uses Simpson's Method to evaluate a definite integral. Function to be integrated must take on parameter of type f32, and return a value of type f32.
/// See the example functions (sin_x and x_cubed) for examples on how to format input.
///
/// # Examples
///
/// ```
/// let calculated_ans1 = srmc::simpsons(&srmc::x_cubed, 2.0, 10.0, 4);
/// let known_ans1 = 2496.0;
///
/// assert_eq!(calculated_ans1, known_ans1);
/// ```
/// Since this method is not exact, in the following example we are checking the function against the expected output, not the mathematically correct value of 2.
/// ```
/// let calculated_ans2 = srmc::simpsons(&srmc::sin_x, 0.0, std::f32::consts::PI, 10);
/// let known_ans2 = 2.0001094;
///
/// assert_eq!(calculated_ans2, known_ans2);
///
pub fn simpsons(f: &dyn Fn(f32) -> f32, lower: f32, upper: f32, steps: i32) -> f32 {
    if lower > upper {
        panic!("Upper bound must be greater than lower bound.");
    }
    if steps % 2 != 0 {
        panic!("Steps must be an even number.")
    }
    let values = create_sequential_vec(lower, upper, steps);
    let mut ans: f32 = 0.0;
    let mut index = 0;
    let max_index = values.len() - 1;
    while index < values.len() {
        if index == 0 || index == max_index {
            ans = ans + f(values[index]);
            index = index + 1;
        } else if index % 2 != 0 {
            ans = ans + f(values[index]) * 4.0;
            index = index + 1;
        } else if index % 2 == 0 {
            ans = ans + f(values[index]) * 2.0;
            index = index + 1;
        }
    }
    ans = (((upper - lower) / steps as f32) * ans) / 3.0;
    return ans;
}

/// Uses The Trapezoidal Rule to evaluate a definite integral. Function to be integrated must take on parameter of type f32, and return a value of type f32.
/// See the example functions (sin_x and x_cubed) for examples on how to format input.
///
/// # Examples
///
/// Integrating the function x cubed from 2.0 to 10.0 with a step size of 10000.
/// ```
/// let calculated_val = srmc::trapzoidal_rule(&srmc::x_cubed, 2.0, 10.0, 10000);
/// ```
/// Integrating the function sin(x) from 2.0 to 10.0 with a step size of 10000.
/// ```
/// let calculated_val = srmc::trapzoidal_rule(&srmc::sin_x, 2.0, 10.0, 10000);
/// ```
pub fn trapzoidal_rule(f: &dyn Fn(f32) -> f32, lower: f32, upper: f32, steps: i32) -> f32 {
    if lower > upper {
        panic!("Upper bound must be greater than lower bound.");
    }
    if steps % 2 != 0 {
        panic!("Steps must be an even number.")
    }
    let values = create_sequential_vec(lower, upper, steps);
    let mut ans: f32 = 0.0;
    let mut index = 0;
    let max_index = values.len() - 1;
    while index < values.len() {
        if index == 0 || index == max_index {
            ans = ans + f(values[index]);
            index = index + 1;
        } else {
            ans = ans + f(values[index]) * 2.0;
            index = index + 1;
        }
    }
    ans = ((upper - lower) / (2.0 * steps as f32)) * ans;
    return ans;
}

/// Function to calculate the relative error between two values
///
/// # Example
///
/// ```
/// let known_ans = 2.0;
/// let calculated_ans = srmc::simpsons(&srmc::sin_x, 0.0, std::f32::consts::PI, 10);
/// let rel_error = srmc::relative_error(calculated_ans, known_ans);
/// println!("Relative error between known and calculated value is: {}", rel_error);
/// ```
///  
pub fn relative_error(observed_val: f32, true_val: f32) -> f32 {
    return ((observed_val - true_val).abs()) / true_val;
}

/// Function to calculate the percentage error between two values
///
/// # Example
///
/// ```
/// let known_ans = 2.0;
/// let calculated_ans = srmc::simpsons(&srmc::sin_x, 0.0, std::f32::consts::PI, 10);
/// let percentage_error = srmc::percentage_error(calculated_ans, known_ans);
/// println!("Percentage error between known and calculated value is: {}%", percentage_error);
/// ```
///  
pub fn percentage_error(observed_val: f32, true_val: f32) -> f32 {
    return (((observed_val - true_val).abs()) / true_val) * 100.0;
}

/// Example input function to simpons for the function x cubed
pub fn x_cubed(x: f32) -> f32 {
    return f32::powf(x, 3.0);
}

/// Example input function for simpsons for the function sin(x)
pub fn sin_x(x: f32) -> f32 {
    return x.sin();
}
