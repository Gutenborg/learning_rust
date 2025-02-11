/** Enums allow you to define a type that can have variants */
enum IpAddrKind {
    V4,
    V6,
}

/** Structs can have enums as members */
struct IpAddrStruct {
    address: String,
    kind: IpAddrKind,
}

/** Enums can enforce types for its members and capture data this way
 * Enum members can have any type, even other enums
*/
enum IpAddrEnum {
    V4(String),
    V6(String),
}

/** Enum members can have different types */
enum IpAddrEnumVariantTypes {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    // We use a struct with one member having the type of an enum to capture associated data for the enum
    let homeStruct = IpAddrStruct {
        address: String::from("127.0.0.1"),
        kind: IpAddrKind::V4,
    };

    // This method is verbose and we can actually capture associated data within the enum itself
    let loopbackStruct = IpAddrStruct {
        address: String::from("::1"),
        kind: IpAddrKind::V6,
    };

    // This enum has the ability to capture type-safe data that is represented as an enum type
    let homeEnum = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopbackEnum = IpAddrEnum::V6(String::from("::1"));

    // Different members of the enum can have different types and are still valid representations of the enum
    let homeEnumVariantType = IpAddrEnumVariantTypes::V4(127, 0, 0, 1);
    let loopbackEnumVariantType = IpAddrEnumVariantTypes::V6(String::from("::1"));

    plus_one(None);
    plus_one(Some(5));
}

/** Takes in a type of an enum, so it matches any variant within the enum */
fn route(ip_kind: IpAddrKind) {}

// A function cannot easily define an argument that can accept multiple data types, but an enum lets us do that
// fn structVariants(structInstance: Struct1 | Struct2) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/** You can also implement an enum */
impl Message {
    /** Self is the value assigned to the enum instance */
    fn call(&self) {
        // Enums can have methods
    }
}

fn enumTest() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // Match expressions are similar to if expressions, but can compare any type not just booleans
    return match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
}

// Match can be used for more than just enums
fn plus_one(x: Option<i32>) -> i32 {
    return match x {
        None => 0,
        Some(i) => i + 1,
    };
}
