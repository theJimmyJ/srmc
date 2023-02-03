fn main() {
    /* Small example of using the srmc */

    // Create a function to be integrated in a way that will work with simpsons function
    fn sinc_x(x: f32) -> f32 {
        return x.sin() / x;
    }

    // Calculate integral from 0 to 2 for sinc_x with simpsons function
    // Since our function sinc_x involves dividing by our input, we start at a value close to zero
    let calculated_val = srmc::simpsons(&sinc_x, 0.001, 2.0, 10000);
    println!("Integral from 0 to 2 of sinc(x): {}", calculated_val);

    // Value of the integral as given by https://www.wolframalpha.com/
    let real_val: f32 = 1.60441;

    // Calculate the relative and percentage error for our value
    let rel_error = srmc::relative_error(calculated_val, real_val);
    let per_error = srmc::percentage_error(calculated_val, real_val);
    println!(
        "Relative error: {} \nPercentage error: {}%",
        rel_error, per_error
    );
}
