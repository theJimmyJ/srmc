fn main() {
    /* Small example of using the srmc */

    println!("Example of using SRMC to calculate the integral of sinc(x) from 2 to 10");

    // Calculate integral from 2 to 10 for sinc_x with simpsons function
    let calculated_val1 = srmc::simpsons(&sinc_x, 2.0, 10.0, 10000);
    println!("With Simpsons Method:  {}", calculated_val1);

    // Calculate integral from 2 to 10 for sinc_x with the trapezoidal rule
    let calculated_val2 = srmc::trapzoidal_rule(&sinc_x, 2.0, 10.0, 10000);
    println!("With Trapezoidal Rule: {}", calculated_val2);

    // Value of the integral as given by https://www.wolframalpha.com/
    let real_val: f32 = 0.0529346;
    println!("Value according to wolfram alpha: {}", real_val);

    // Find the relative error for each method
    let rel_error_simpsons = srmc::relative_error(calculated_val1, real_val);
    let rel_error_trapezoidal = srmc::relative_error(calculated_val2, real_val);

    println!(
        "Relative Error for Simpsons Method:  {}",
        rel_error_simpsons
    );
    println!(
        "Relative Error for Trapezoidal Rule: {}",
        rel_error_trapezoidal
    );
}

// Create a function to be integrated in a way that will work with simpsons function
fn sinc_x(x: f32) -> f32 {
    return x.sin() / x;
}
