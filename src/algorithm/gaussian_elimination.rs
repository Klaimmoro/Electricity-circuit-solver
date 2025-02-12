pub struct GaussianElimination {
    value: Vec<f64>
}
//
//
impl GaussianElimination {
    ///
    /// Struct constructor
    pub fn new() -> Self {
        Self { 
            value: Vec::new()
        }
    }
    ///
    /// Method to eliminate
    pub fn gaussian_elimination(matrix: &mut [Vec<f32>]) -> Vec<f32> {
        let size = matrix.len();
        assert_eq!(size, matrix[0].len() - 1);
    
        for i in 0..size - 1 {
            for j in i..size - 1 {
                Self::echelon(matrix, i, j);
            }
        }
    
        for i in (1..size).rev() {
            Self::eliminate(matrix, i);
        }
    
        // Disable cargo clippy warnings about needless range loops.
        // Checking the diagonal like this is simpler than any alternative.
        #[allow(clippy::needless_range_loop)]
        for i in 0..size {
            if matrix[i][i] == 0f32 {
                println!("Infinitely many solutions");
            }
        }
    
        let mut result: Vec<f32> = vec![0f32; size];
        for i in 0..size {
            result[i] = matrix[i][size] / matrix[i][i];
        }
        result
    }
    ///
    /// 
    fn echelon(matrix: &mut [Vec<f32>], i: usize, j: usize) {
        let size = matrix.len();
        if matrix[i][i] == 0f32 {
        } else {
            let factor = matrix[j + 1][i] / matrix[i][i];
            (i..=size).for_each(|k| {
                matrix[j + 1][k] -= factor * matrix[i][k];
            });
        }
    }
    ///
    /// 
    fn eliminate(matrix: &mut [Vec<f32>], i: usize) {
        let size = matrix.len();
        if matrix[i][i] == 0f32 {
        } else {
            for j in (1..=i).rev() {
                let factor = matrix[j - 1][i] / matrix[i][i];
                for k in (0..=size).rev() {
                    matrix[j - 1][k] -= factor * matrix[i][k];
                }
            }
        }
    }
}