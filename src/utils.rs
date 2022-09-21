use sfml::system::{Vector2f, Vector2i};

pub fn vector2f_to_vector2i(vector: Vector2f) -> Vector2i{
    Vector2i::new(vector.x as i32, vector.y as i32)
}