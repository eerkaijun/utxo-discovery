// Helper function to multiply two matrices
pub fn matrix_multiply(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();

    // Initialize the result matrix with zeros
    let mut result = vec![vec![0 as f64; cols_b]; rows_a];

    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    result
}

// Helper function to perform addition of two matrices of same dimension
pub fn matrix_add(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    let len = a.len();
    
    // Initialize the result matrix with zeros
    let mut result = vec![0 as f64; len];

    for i in 0..len {
        result[i] = a[i] + b[i];
    }

    result
}

// Helper function to transpose a column vector in a row vector
pub fn transpose(matrix: &Vec<Vec<f64>>) -> Vec<f64> {
    let mut result = Vec::new();

    for row in matrix {
        for &element in row {
            result.push(element);
        }
    }

    result
}