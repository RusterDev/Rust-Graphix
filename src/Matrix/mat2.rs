pub trait Op {
    fn add(&mut self, other: Matrix2) -> Matrix2;
    fn sub(&mut self, other: Matrix2) -> Matrix2;
    fn scalar_multiply(&mut self, scalar: i64) -> Matrix2;
    fn multiply_matrices(&mut self, other: Matrix2) -> Matrix2;
    fn transpose(&mut self) -> Matrix2;
    fn determinant(&self) -> i64;
    fn cofactor_matrix(&self) -> Matrix2;
    fn inverse_matrix(&self) -> Option<Matrix2>;
    fn print_matrix(&self);
}

impl Op for Matrix2 {
    fn add(&mut self, other: Matrix2) -> Matrix2 {
        [
            [self[0][0] + other[0][0], self[0][1] + other[0][1]],
            [self[1][0] + other[1][0], self[1][1] + other[1][1]],
        ]
    }

    fn sub(&mut self, other: Matrix2) -> Matrix2 {
        [
            [self[0][0] - other[0][0], self[0][1] - other[0][1]],
            [self[1][0] - other[1][0], self[1][1] - other[1][1]],
        ]
    }

    fn scalar_multiply(&mut self, scalar: i64) -> Matrix2 {
        [
            [self[0][0] * scalar, self[0][1] * scalar],
            [self[1][0] * scalar, self[1][1] * scalar],
        ]
    }

    fn multiply_matrices(&mut self, other: Matrix2) -> Matrix2 {
        [
            [
                self[0][0] * other[0][0] + self[0][1] * other[1][0],
                self[0][0] * other[0][1] + self[0][1] * other[1][1],
            ],
            [
                self[1][0] * other[0][0] + self[1][1] * other[1][0],
                self[1][0] * other[0][1] + self[1][1] * other[1][1],
            ],
        ]
    }

    fn transpose(&mut self) -> Matrix2 {
        [
            [self[0][0], self[1][0]],
            [self[0][1], self[1][1]],
        ]
    }

    fn determinant(&self) -> i64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    fn cofactor_matrix(&self) -> Matrix2 {
        [
            [self[1][1], -self[1][0]],
            [-self[0][1], self[0][0]],
        ]
    }

    fn inverse_matrix(&self) -> Option<Matrix2> {
        let det = self.determinant();
        if det == 0 {
            None
        } else {
            let cofactor = self.cofactor_matrix().transpose();
            Some(cofactor.scalar_multiply(1 / det))
        }
    }

    fn print_matrix(&self) {
        for row in self.iter() {
            println!("{:?}", row);
        }
    }
}
