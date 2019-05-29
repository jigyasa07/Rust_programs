enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let both_integer = Point { x: 25, y: 1 };
    let integer_and_float = Point { x: 34.23, y: 40 };
    let both_float = Point { x: 25.43, y: 4.02 };

     println!("both_integer.x = {},both_integer.y = {}", both_integer.x,both_integer.y);
}
