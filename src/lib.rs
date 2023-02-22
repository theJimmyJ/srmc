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

/// Example input function for x cubed
pub fn x_cubed(x: f32) -> f32 {
    return f32::powf(x, 3.0);
}

/// Example input function for sin(x)
pub fn sin_x(x: f32) -> f32 {
    return x.sin();
}

/// SRMC Matrix struct with various implementations
///
/// Note: When indexing a value or row/column of a Matrix, the numbering starts at zero. So for a 3x3 Matrix,
/// the top row is the 0th row and the top left value is at position (0, 0).
///
/// It is possible to create a matrix that is invalid by having rows of different lengths, there are no checks to ensure
/// that this is not the case when creating a new Matrix. If you're creating a Matrix directly from a vector of vectors and manually
/// entering the dimensions, ensure all values are entered properly. Alternatively, there is a "create_matrix" function and
/// "change_row"/"change_column" implemenations with checks to ensure a matrix is created/changed correctly. To check if a Matrix
/// was made correctly use the "is_valid" implementation.
///
/// Example of creating a Matrix:
/// ```
/// let input: Vec<Vec<f32>> = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
/// let matrix = srmc::Matrix {
///    elements: input,
///    rows: 2,
///    columns: 2,
///};
/// assert_eq!(true, matrix.is_valid());
/// ```
pub struct Matrix {
    pub elements: Vec<Vec<f32>>,
    pub rows: i32,
    pub columns: i32,
}

impl Matrix {
    /// Prints matrix in terminal
    pub fn print(&self) {
        for item in &self.elements {
            println!("{:?}", item);
        }
    }
    /// Returns the dimensions of the matrix as a tuple (rows, columns)
    pub fn get_dimensions(&self) -> (i32, i32) {
        return (self.rows, self.columns);
    }

    /// Checks if a Matrix has rows of equal length and correct values for number of rows and columns
    pub fn is_valid(&self) -> bool {
        let row_length = self.elements[0].len();
        let mut i: usize = 1;
        while i < self.elements.len() {
            if self.elements[i].len() != row_length {
                return false;
            }
            i = i + 1;
        }
        if self.elements.len() != self.rows as usize {
            return false;
        }
        if self.elements[0].len() != self.columns as usize {
            return false;
        }
        return true;
    }

    /// Returns the element of the matrix at the given position
    pub fn index_val(&self, row: i32, column: i32) -> f32 {
        if &(row + 1) > (&self.rows) || &(column + 1) > (&self.columns) {
            panic!("Row and Column position outside the bounds matrix. Please provide a value for rows from 0 to {} and a value for columns from 0 to {}", self.rows - 1, self.columns - 1);
        }
        let row_num = &self.elements[row as usize];
        return row_num[column as usize];
    }

    /// Changes the element of the matrix at the given position
    pub fn change_val(&mut self, val: f32, row: i32, column: i32) {
        if &(row + 1) > (&self.rows) || &(column + 1) > (&self.columns) {
            panic!("Row and Column position outside the bounds matrix. Please provide a value for rows from 0 to {} and a value for columns from 0 to {}", self.rows - 1, self.columns - 1);
        }
        self.elements[row as usize][column as usize] = val;
    }

    /// Multiplies entire matrix by some value
    pub fn multiply_by_value(&mut self, val: f32) {
        let mut i: usize = 0;
        while i < self.rows as usize {
            let mut j = 0;
            while j < self.columns as usize {
                self.elements[i][j] = self.elements[i][j] * val;
                j = j + 1;
            }
            i = i + 1;
        }
    }

    /// Changes given row to input vector, input vector must match dimensions of the matrix
    ///
    /// Example:
    /// ```
    /// let mut matrix = srmc::create_matrix(2, 3);
    /// let new_row = vec![1.0, 2.0, 3.0];
    /// matrix.change_row(0, new_row);
    /// ```
    pub fn change_row(&mut self, row: i32, new_row: Vec<f32>) {
        if new_row.len() != self.elements[row as usize].len() {
            panic!("Input vector not matching matrix dimensions.");
        }
        self.elements[row as usize] = new_row;
    }

    /// Changes given column to input vector, input vector must match dimensions of the matrix
    ///
    /// Example:
    /// ```
    /// let mut matrix = srmc::create_matrix(2, 3);
    /// let new_column = vec![1.0, 2.0];
    /// matrix.change_column(0, new_column);
    /// ```
    pub fn change_column(&mut self, column: i32, new_column: Vec<f32>) {
        if new_column.len() != self.elements.len() {
            panic!("Input vector not matching matrix dimensions.");
        }
        let mut i: usize = 0;
        while i < self.elements.len() {
            self.elements[i][column as usize] = new_column[i];
            i = i + 1;
        }
    }
}

/// Returns an SRMC Matrix swith provided dimensions. All elements of matrix are initially set to zero.
///
/// Creates 3x3 Matrix
/// ```
/// let matrix = srmc::create_matrix(3, 3);
/// ```
/// Creates 1x5 Matrix
/// ```
/// let matrix = srmc::create_matrix(1, 5); //creates 1x5 matrix
/// ```
pub fn create_matrix(rows: i32, columns: i32) -> Matrix {
    return Matrix {
        elements: vec![vec![0.0; columns as usize]; rows as usize],
        rows,
        columns,
    };
}

/// Returns an SRMC Matrix with provided dimensions. Elements of the matrix start at 1.0 and are incremented by 1.
/// ```
/// let matrix = srmc::create_numbered_matrix(2, 4);
/// matrix.print();
/// // The following matrix is printed when .print() is called:
/// //[1.0, 2.0, 3.0, 4.0]
/// //[5.0, 6.0, 7.0, 8.0]
/// ```
pub fn create_numbered_matrix(rows: i32, columns: i32) -> Matrix {
    let mut num_matrix = create_matrix(rows, columns);
    let mut val: f32 = 0.0;
    let mut i: usize = 0;
    while i < rows as usize {
        let mut j: usize = 0;
        while j < columns as usize {
            val = val + 1.0;
            num_matrix.elements[i][j] = val;
            j = j + 1;
        }
        i = i + 1;
    }
    return num_matrix;
}

/// Returns an SRMC Matrix with diagonal values set to one.
///
/// Crates 3x3 identity matrix
/// ```
/// let matrix = srmc::create_identity_matrix(3);
/// ```
/// Creates 10x10 identity matrix
/// ```
/// let matrix = srmc::create_identity_matrix(10);
/// ```
pub fn create_identity_matrix(dimension: i32) -> Matrix {
    let mut res_mat = create_matrix(dimension, dimension);
    let mut i: usize = 0;
    while i < dimension as usize {
        res_mat.elements[i][i] = 1.0;
        i = i + 1;
    }
    return res_mat;
}

/// Adds two Matricies together and returns the result as an SRMC Matrix
/// ```
/// let matrix1 = srmc::create_numbered_matrix(3,3);
/// let matrix2 = srmc::create_numbered_matrix(3,3);
/// let matrix3 = srmc::matrix_addition(&matrix1, &matrix2);
/// ```
/// NOTE: Running this function keeps input Matricies unchanged and creates a new Matrix.
/// If you want to run without creating a new Matrix do the following:
/// ```
/// let mut matrix1 = srmc::create_numbered_matrix(3,3);
/// let matrix2 = srmc::create_numbered_matrix(3,3);
/// let matrix1 = srmc::matrix_addition(&matrix1, &matrix2);
/// ```
/// In the above code, matrix2 could have also been mutable and set equal to the result.
pub fn matrix_addition(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    if mat1.rows != mat2.rows || mat1.columns != mat2.columns {
        panic!("Matricies must be of the same size to perform matrix addition.");
    }
    let mut res_mat = create_matrix(mat1.rows, mat1.columns);

    let mut i: usize = 0;
    while i < mat1.rows as usize {
        let mut j = 0;
        while j < mat1.columns as usize {
            res_mat.elements[i][j] = mat1.elements[i][j] + mat2.elements[i][j];
            j = j + 1;
        }
        i = i + 1;
    }
    return res_mat;
}

/// Subtracts two Matricies and returns the result as an SRMC Matrix.
///
/// As matricies do not commute, input order is important. For this the function
/// the second argument is subtracted from the first. So in the following code
/// matrix3 = matrix1 - matrix2.
/// ```
/// let matrix1 = srmc::create_numbered_matrix(3,3);
/// let matrix2 = srmc::create_identity_matrix(3);
/// let matrix3 = srmc::matrix_subtraction(&matrix1, &matrix2);
/// ```
/// NOTE: Running this function keeps input Matricies unchanged and creates a new Matrix.
/// If you want to run without creating a new Matrix do the following:
/// ```
/// let mut matrix1 = srmc::create_numbered_matrix(3,3);
/// let matrix2 = srmc::create_identity_matrix(3);
/// matrix1 = srmc::matrix_subtraction(&matrix1, &matrix2);
/// ```
/// In the above code, matrix2 could have also been mutable and set equal to the result.
pub fn matrix_subtraction(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    if mat1.rows != mat2.rows || mat1.columns != mat2.columns {
        panic!("Matricies must be of the same size to perform matrix subtraction.");
    }
    let mut res_mat = create_matrix(mat1.rows, mat1.columns);

    let mut i: usize = 0;
    while i < mat1.rows as usize {
        let mut j = 0;
        while j < mat1.columns as usize {
            res_mat.elements[i][j] = mat1.elements[i][j] - mat2.elements[i][j];
            j = j + 1;
        }
        i = i + 1;
    }
    return res_mat;
}

/// Returns the tranpose of an SRMC Matrix
///
/// Example Code:
/// ```
/// let matrix = srmc::create_numbered_matrix(2,4);
/// let matrix2 = srmc::matrix_transpose(&matrix);
/// ```
/// NOTE: Running this function keeps input Matrix unchanged and creates a new Matrix.
/// If you want to change input Matrix without creating a new one do the following:
/// ```
/// let mut matrix = srmc::create_numbered_matrix(2,4);
/// matrix = srmc::matrix_transpose(&matrix);
///```
pub fn matrix_transpose(mat: &Matrix) -> Matrix {
    let mut res_mat = create_matrix(mat.columns, mat.rows);
    let mut i: usize = 0;
    while i < mat.columns as usize {
        let mut j: usize = 0;
        while j < mat.rows as usize {
            res_mat.elements[i][j] = mat.elements[j][i];
            j = j + 1;
        }
        i = i + 1;
    }
    return res_mat;
}
