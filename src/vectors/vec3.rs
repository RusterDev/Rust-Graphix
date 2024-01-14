pub mod Vector3 {
    pub trait Props {}

    pub trait Op {
        fn add(&mut self, vector3: Vector3);
        fn sub(&mut self, other: Vector3) -> Vector3;
        fn dot(&self, vector3: Vector3) -> f64;
    }

    pub trait Transform {
        fn translate(&mut self, x: f64, y: f64, z: f64);
        fn normalize(&mut self);

    }

    pub trait Analysis {
        fn Distance(&mut self, vector: Vector3) -> f64;
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub magnitude: f64,
    }

    impl Vector3 {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Vector3 {
                x: x,
                y: y,
                z: z,
                magnitude: f64::sqrt(x * x + y * y + z * z),
            }
        }
    }

    impl Op for Vector3 {
        fn add(&mut self, vector: Vector3) {
            self.x += vector.x;
            self.y += vector.y;
            self.z += vector.z;
        }

        fn sub(&mut self, other: Vector3) -> Vector3 {
            Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)

        }

        fn dot(&self, vector: Vector3) -> f64 {
            self.x * vector.x + self.y * vector.y + self.z * vector.z
        }
    }

    impl Transform for Vector3 {
        fn translate(&mut self, x: f64, y: f64, z: f64) {
            self.x += x;
            self.y += y;
            self.z += z;
        }
        fn normalize(&mut self) {
            self.x/=self.magnitude;
            self.y/=self.magnitude;
            self.z/=self.magnitude;
            self.magnitude=1.0;
        }
    }
    impl Analysis for Vector3 {
        fn Distance(&mut self, vector: Vector3) -> f64 {
          let mut res = self.sub(vector).magnitude;
          return res;

        }
    }
}
