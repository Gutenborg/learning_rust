// Set up the "Debug" trait
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(self: &Self) -> u32 {
        return self.height * self.width;
    }

    fn width(&self) -> bool {
        return self.width > 0;
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        if self.height > other_rect.height && self.width > other_rect.width {
            return true;
        } else {
            return false;
        }
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

fn main() {
    // Variables
    let width1 = 30;
    let height1 = 50;

    print_area(area_variables(height1, width1));

    // Tuple
    let rect1 = (30, 50);

    print_area(area_tuple(rect1));

    // Struct
    let rect2 = Rectangle {
        height: 50,
        width: 30,
    };

    let rect3 = Rectangle {
        height: 40,
        width: 20,
    };

    let rect4 = Rectangle::square(40);

    // Using a defined function
    print_area(area_struct(&rect2));

    // Using a method on the struct
    print_area(rect2.area());

    // Checking to see if one rectangle can hold another one
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));

    if rect2.width() {
        println!("The rectangle has a non zero width; it is {}", rect2.width);
    }

    // Debug Struct
    // Fails because it doesn't have a formatter
    // println!("rect2 is {}", rect2);

    // Succeeds because we are deriving the "Debug" trait
    println!("rect2 is {rect2:#?}");

    // Debug can take a reference
    dbg!(&rect2);

    // Debug can also take ownership and return it
    let rect2 = dbg!(rect2);
    print_area(area_struct(&rect2));
}

fn print_area(area: u32) {
    println!("The area of the rectangle is {} square pixels", area);
}

fn area_variables(width: u32, height: u32) -> u32 {
    return width * height;
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    return rectangle.height * rectangle.width;
}
