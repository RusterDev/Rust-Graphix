pub type Matrix3 = [[[f64; 3]; 3]; 3];

pub trait Op3 {
    fn add_3(&mut self, other: Matrix3) -> Matrix3;
    fn sub_3(&mut self, other: Matrix3) -> Matrix3;
    fn scalar_multiply_3(&mut self, scalar: f64) -> Matrix3;
    fn multiply_matrices_3(&mut self, other: Matrix3) -> Matrix3;
    fn transpose_3(&mut self) -> Matrix3;
    fn determinant_3(&self) -> f64;
    fn cofactor_matrix_3(&self) -> Matrix3;
    fn inverse_matrix_3(&self) -> Option<Matrix3>;
    fn print_matrix_3(&self);
}

impl Op3 for Matrix3 {
    fn add_3(&mut self, other: Matrix3) -> Matrix3 {
        let mut result = [[[0.0; 3]; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result[i][j][k] = self[i][j][k] + other[i][j][k];
                }
            }
        }
        result
    }

    fn sub_3(&mut self, other: Matrix3) -> Matrix3 {
        let mut result = [[[0.0; 3]; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result[i][j][k] = self[i][j][k] - other[i][j][k];
                }
            }
        }
        result
    }

    fn scalar_multiply_3(&mut self, scalar: f64) -> Matrix3 {
        let mut result = [[[0.0; 3]; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result[i][j][k] = self[i][j][k] * scalar;
                }
            }
        }
        result
    }

    fn multiply_matrices_3(&mut self, other: Matrix3) -> Matrix3 {
        let mut result = [[[0.0; 3]; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        result[i][j][k] += self[i][j][l] * other[l][k][j];
                    }
                }
            }
        }
        result
    }

    fn transpose_3(&mut self) -> Matrix3 {
        let mut result = [[[0.0; 3]; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                result[i][j] = self[j][i];
            }
        }
        result
    }

    fn determinant_3(&self) -> f64 {
        0.0
    }

    fn cofactor_matrix_3(&self) -> Matrix3 {
        [[[0.0; 3]; 3]; 3]
    }

    fn inverse_matrix_3(&self) -> Option<Matrix3> {
        Some([[[0.0; 3]; 3]; 3])
    }

    fn print_matrix_3(&self) {
        for i in 0..3 {
            for j in 0..3 {
                println!("{:?}", self[i][j]);
            }
        }
    }
}
