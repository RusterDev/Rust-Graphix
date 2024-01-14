pub(crate) mod vectors;
use vectors::vec3::Vector3::Vector3;

fn main() {
    let vec_: Vector3 = Vector3::new(1.0, 1.0, 1.0);
    println!("vec: {:?}", vec_);

}
