struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

#[derive(Debug)]
struct ComplexPoint<T, U> {
    x: T,
    y: U,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> ComplexPoint<T, U> {
    fn mixup<V, W>(self, other: ComplexPoint<V, W>) -> ComplexPoint<T, W> {
        ComplexPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest_int(&number_list));

    let char_list = vec!['a', 'j', 'e', 'w', '!'];

    println!("The largest char is {}", largest_char(&char_list));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("x is {}, y is {}", integer.x(), integer.y());
    println!("distance from origin: {}", float.distance_from_origin());

    let complex1 = ComplexPoint { x: 1, y: 5.0 };
    let complex2 = ComplexPoint { x: 'a', y: true };
    let complex3 = complex1.mixup(complex2);

    println!("FUSIOOON {:?}", complex3);
}

fn largest_int(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for c in list {
        if c > largest {
            largest = c;
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
