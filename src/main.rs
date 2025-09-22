fn main() {
    println!("Hello, world!");
    println!("Hello, sigit!");
    println!("Hello, priadi!");
    println!("Hello, sigit priadi!");
}

// #[test] --> make unit test, and it is says attribute

// make unit test hello world
#[test]
fn test_hello() {
    println!("Hello, world!");
}

// make unit test variable
#[test] // test variable immutable
fn test_variable_immutable() {
   let name = "sigit";
    println!("Hello, {}", name);
}

#[test] // test variable mutable
fn test_variable_mutable() {
    let mut name = "sigit";
    println!("Hello, {}", name);
    name = "sigit priadi";
    println!("Hello, {}", name);
}

#[test] // test variable shadowing, this is very bad practice, but it is possible
fn test_variable_shadowing() {
    let name = "sigit";
    println!("Hello, {}", name);
    let name = 2024;
    println!("Hello, {}", name);
}

/*
make unit test function for data type
1. Scalar type
    - Integer
    - Floating-point
    - Boolean
    - Character
2. Compound type
    - Tuple
    - Array
3. Explicit type
    - Explicit type
    - Inference type
*/
#[test]
fn test_scalar_type() {
    println!("====================================");

    // Integer
    let integer = 2024;
    println!("Integer: {}", integer);
    let integer8: i8 = 8;
    println!("Integer 8: {}", integer8);
    let uinteger8: u8 = 8;
    println!("Unsigned Integer 8: {}", uinteger8);
    let integer16: i16 = 16;
    println!("Integer 16: {}", integer16);
    let uinteger16: u16 = 16;
    println!("Unsigned Integer 16: {}", uinteger16);
    let integer32: i32 = 32;
    println!("Integer 32: {}", integer32);
    let uinteger32: u32 = 32;
    println!("Unsigned Integer 32: {}", uinteger32);
    let integer64: i64 = 64;
    println!("Integer 64: {}", integer64);
    let uinteger64: u64 = 64;
    println!("Unsigned Integer 64: {}", uinteger64);
    let integer128: i128 = 128;
    println!("Integer 128: {}", integer128);
    let uinteger128: u128 = 128;
    println!("Unsigned Integer 128: {}", uinteger128);
    let integer_size: isize = 2024;
    println!("Integer Size: {}", integer_size);
    let uinteger_size: usize = 2024;
    println!("Unsigned Integer Size: {}", uinteger_size);

    println!("====================================");

    // Floating-point
    let float = 3.14;
    println!("Float: {}", float);
    let float32: f32 = 3.14;
    println!("Float 32: {}", float32);
    let float64: f64 = 3.14;
    println!("Float 64: {}", float64);

    println!("====================================");

    // Boolean
    let boolean = true;
    println!("Boolean: {}", boolean);

    println!("====================================");

    // Character
    let character = 'A';
    println!("Character: {}", character);

    println!("====================================");
}

#[test]
fn test_compound_type() {
    // Tuple
    let tuple = (2024, 3.14, true, 'A');
    println!("Tuple: {:?}", tuple);

    // Array char
    let array = ['A', 'B', 'C'];
    println!("Array: {:?}", array);
}

#[test]
fn test_explicit_type() {
    // Explicit type
    let integer: i32 = 2024;
    println!("Integer: {}", integer);

    // Inference type
    let integer = 2024;
    println!("Integer: {}", integer);
}

#[test]
fn test_number_conversion() {
    // Integer to Integer
    let integer = 2024;
    let integer8 = integer as i8;
    println!("Integer to Integer 8: {}", integer8);
    let integer16 = integer as i16;
    println!("Integer to Integer 16: {}", integer16);
    let integer64 = integer as i64;
    println!("Integer to Integer 64: {}", integer64);

    // Integer to float
    let integer = 2024;
    let float = integer as f64;
    println!("Integer to Float: {}", float);

    // Float to integer
    let float = 3.14;
    let integer = float as i32;
    println!("Float to Integer: {}", integer);
}

// make unit test function for operator
#[test]
fn test_operator() {
    // Arithmetic operator
    let a = 10;
    let b = 5;
    let addition = a + b;
    println!("Addition: {}", addition);
    let subtraction = a - b;
    println!("Subtraction: {}", subtraction);
    let multiplication = a * b;
    println!("Multiplication: {}", multiplication);
    let division = a / b;
    println!("Division: {}", division);
    let remainder = a % b;
    println!("Remainder: {}", remainder);

    // Comparison operator
    let a = 10;
    let b = 5;
    let equal = a == b;
    println!("Equal: {}", equal);
    let not_equal = a != b;
    println!("Not Equal: {}", not_equal);
    let greater_than = a > b;
    println!("Greater Than: {}", greater_than);
    let less_than = a < b;
    println!("Less Than: {}", less_than);
    let greater_than_or_equal = a >= b;
    println!("Greater Than or Equal: {}", greater_than_or_equal);
    let less_than_or_equal = a <= b;
    println!("Less Than or Equal: {}", less_than_or_equal);

    // Logical operator
    let a = true;
    let b = false;
    let and = a && b;
    println!("And: {}", and);
    let or = a || b;
    println!("Or: {}", or);
    let not = !a;
    println!("Not: {}", not);
}

// make unit test function for augmented assignment
#[test]
fn test_augmented_assignment() {
    let mut a = 10;
    let b = 5;
    a += b;
    println!("Addition: {}", a);
    a -= b;
    println!("Subtraction: {}", a);
    a *= b;
    println!("Multiplication: {}", a);
    a /= b;
    println!("Division: {}", a);
    a %= b;
    println!("Remainder: {}", a);
}

// make unit test function for comparison operator
#[test]
fn test_comparison_operator() {
    let a = 10;
    let b = 5;
    let equal = a == b;
    println!("Equal: {}", equal);
    let not_equal = a != b;
    println!("Not Equal: {}", not_equal);
    let greater_than = a > b;
    println!("Greater Than: {}", greater_than);
    let less_than = a < b;
    println!("Less Than: {}", less_than);
    let greater_than_or_equal = a >= b;
    println!("Greater Than or Equal: {}", greater_than_or_equal);
    let less_than_or_equal = a <= b;
    println!("Less Than or Equal: {}", less_than_or_equal);
}

// make unit test function for character
#[test]
fn test_character() {
    let character = 'A';
    println!("Character: {}", character);
    let character = '😀';
    println!("Character: {}", character);
    let character: char = '\u{1F600}';
    println!("Character: {}", character);
    // let character: char = 'TEST'; // error: character literal may only contain one codepoint
}

// make unit test function for tuple
#[test]
fn test_tuple() {
    let tuple = (2024, 3.14, true, 'A');
    println!("Tuple: {:?}", tuple);
    let tuple = (2024, 3.14, true, 'A');
    let (integer, float, boolean, character) = tuple;
    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    let tuple = (2024, 3.14, true, 'A');
    let integer = tuple.0;
    let float = tuple.1;
    let boolean = tuple.2;
    let character = tuple.3;
    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    // by default, tuple is immutable
    // lets try to change tuple to mutable
    let mut tuple = (2024, 3.14, true, 'A');
    tuple.0 = 2025;
    tuple.1 = 3.15;
    tuple.2 = false;
    tuple.3 = 'B';
    println!("Tuple: {:?}", tuple);
}

// make unit test function tuple unit default
fn tuple_unit() {
    println!("Tuple Unit");
}
#[test]
fn test_tuple_unit() {
    let result = tuple_unit();
    println!("Result: {:?}", result);

    let test = ();
    println!("Test: {:?}", test);
}

// make unit test function for array
#[test]
fn test_array() {
    let array = ['A', 'B', 'C'];
    println!("Array: {:?}", array);
    let array = ['A', 'B', 'C'];
    let element = array[0];
    println!("Element: {}", element);
    let array = ['A', 'B', 'C'];
    let length = array.len();
    println!("Length: {}", length);
    let array = ['A', 'B', 'C'];
    let slice = &array[0..2];
    println!("Slice: {:?}", slice);
    let array = ['A', 'B', 'C'];
    let slice = &array[0..];
    println!("Slice: {:?}", slice);
    let array = ['A', 'B', 'C'];
    let slice = &array[..2];
    println!("Slice: {:?}", slice);
    let array = ['A', 'B', 'C'];
    let slice = &array[..];
    println!("Slice: {:?}", slice);
    // by default, array is immutable
    // lets try to change array to mutable
    let mut array = ['A', 'B', 'C'];
    array[0] = 'D';
    array[1] = 'E';
    array[2] = 'F';
    println!("Array: {:?}", array);
}

// make unit test function for array dimension
#[test]
fn test_array_dimension() {
   let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("Matrix: {:?}", matrix);
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let element = matrix[0][0];
    println!("Element: {}", element);
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let length = matrix.len();
    println!("Length: {}", length);
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let slice = &matrix[0..2];
    println!("Slice: {:?}", slice);
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let slice = &matrix[0..];
    println!("Slice: {:?}", slice);
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let slice = &matrix[..2];
    println!("Slice: {:?}", slice);
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let slice = &matrix[..];
    println!("Slice: {:?}", slice);
    // by default, array is immutable
    // lets try to change array to mutable
    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    matrix[0][0] = 10;
    matrix[0][1] = 20;
    matrix[0][2] = 30;
    matrix[1][0] = 40;
    matrix[1][1] = 50;
    matrix[1][2] = 60;
    matrix[2][0] = 70;
    matrix[2][1] = 80;
    matrix[2][2] = 90;
    println!("Matrix: {:?}", matrix);
}

// make unit test function for constant
#[test]
fn test_constant() {
    const PI: f64 = 3.14;
    println!("PI: {}", PI);
    const G: f64 = 9.8;
    println!("G: {}", G);

    const PI_F32: f32 = 3.14;
    println!("PI F32: {}", PI_F32);

    const G_F32: f32 = 9.8;
    println!("G F32: {}", G_F32);
}

// make unit test function for variable scope
const MAXIMUM: i32 = 100;

#[test]
fn test_variable_scope() {
    const MINIMUM: i32 = 0;
    println!("MINIMUM: {}", MINIMUM);
    println!("MAXIMUM: {}", MAXIMUM);

    let sigit = 1;

    // inner scope
    {
        let priadi = 2;
        println!("Outer Sigit: {}", sigit);
        println!("Inner Priadi: {}", priadi);
    }

    // println!("Inner Priadi: {}", priadi); // error: cannot find value `priadi` in this scope
}

#[test]
fn test_variable_scope_error() {
    // println!("MINIMUM: {}", MINIMUM); // error: cannot find value `MINIMUM` in this scope
    println!("MAXIMUM: {}", MAXIMUM);
}

// make unit test function for management memory
fn function_a() {
    let a = 10;
    let b = String::from("sigit");
    println!("function a => a: {}, b: {}", a, b);
}

fn function_b() {
    let a = 20;
    let b = String::from("priadi");
    println!("function b => a: {}, b: {}", a, b);
}

#[test]
fn test_management_memory() {
    function_a();
    function_b();
}

#[test]
fn string(){
    // this is string slice, immutable, save in stack
    let name: &str = "  sigit priadi  ";
    let trim: &str = name.trim();
    println!("Original: '{}'", name); // can't edit name because &str is immutable. this is slice, and this is immut, save in stack
    println!("Trimmed: '{}'", trim);

    let mut username: &str = "  sigit priadi  ";
    username = username.trim();
    println!("Username: '{}'", username); // this is possible because we change the reference of username, not the value
}

#[test]
fn string_mutable(){
    // this is string, mutable, save in heap
    let mut name: String = String::from("sigit priadi");
    name.push_str(" - 2024");
    println!("Name: {}", name); // ini mengubah value dari name, karena String ini mutable dan disimpan di heap

    let test = name.push_str(" - test");
    println!("Test: {:?}", test); // ini mengembalikan unit type, karena push_str ini tidak mengembalikan apapun, jadi dia mengembalikan unit type

    let budi = name.replace("sigit", "budi");
    println!("Budi: {}", budi); // ini tidak mengubah value dari name, karena replace ini membuat string baru, jadinya menambah heap baru
}

#[test]
fn function_string() {
    let name1 = "Sigit";
    println!("Hello, {}", name1); // this is string slice, immutable, save in stack

    let name2 = String::from("Priadi");
    println!("Hello, {}", name2); // this is string, mutable, save in heap
}

#[test]
fn ownership_rules() {
    // a tidak bisa diakses, belum di deklarasikan
    let a = 10; // a bisa diakses mulai dari sini

    {
        // b tidak bisa diakses, belum di deklarasikan
        let b = 20; // b bisa diakses mulai dari sini
        println!("b: {}", b);
    } // b tidak bisa diakses, sudah keluar dari scope dan sudah di drop

    println!("a: {}", a); // a bisa diakses sampai sini
} // a tidak bisa diakses, sudah keluar dari scope dan sudah di drop

#[test]
fn data_copy() {
    // ini hanya berlaku tipe data yang bersifat fixed size, seperti integer, float, boolean, char, str slice, dan tuple yang isinya hanya tipe data fixed size
    let a = 10;
    let b = a; // a di copy ke b, karena i32 ini implementasi trait Copy
    println!("a: {}, b: {}", a, b); // a masih bisa diakses

    let c = 10;
    let mut d = c; // c di copy ke d, karena i32 ini implementasi trait Copy
    d += 20; // d diubah nilainya, tidak mempengaruhi c. jika d = 20, maka itu hanya mengubah referensi d, bukan mengubah nilai d
    println!("c: {}, d: {}", c, d); // c masih bisa diakses
}

#[test]
fn ownership_movement() {
    // ini berlaku untuk tipe data yang bersifat dynamic size, seperti String, Vec, HashMap, dan lain-lain
    let name1 = String::from("sigit priadi");
    // ownership dari name1 di pindahkan ke name2
    let name2 = name1; // name1 tidak bisa diakses lagi, karena ownership dari name1 sudah di pindahkan ke name2
    // println!("name1: {}", name1); // error: borrow of moved value: `name1`
    println!("name2: {}", name2); // name2 bisa diakses
}

#[test]
fn ownership_clone() {
    // ini berlaku untuk tipe data yang bersifat dynamic size, seperti String, Vec, HashMap, dan lain-lain
    let name1 = String::from("sigit priadi");
    // ownership dari name1 di clone ke name2
    let name2 = name1.clone(); // name1 masih bisa diakses, karena ownership dari name1 di clone ke name2
    // perlu diingat, proses clone ini memakan waktu dan memory, karena harus mengcopy semua data dari heap
    println!("name1: {}", name1); // name1 masih bisa diakses
    println!("name2: {}", name2); // name2 bisa diakses
}

#[test]
fn if_expression() {
    let number = 10;

    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    let result: &str = if number % 2 == 0 {
        "even"
    } else {
        "odd"
    };

    println!("{} is {}", number, result);
}

#[test]
fn if_expression_2() {
    let number: i32 = 10;

    if number % 2 == 0 {
        println!("{} is even", number);
    } else if number % 3 == 0 {
        println!("{} is multiple of 3", number);
    } else {
        println!("{} is odd", number);
    }
}

#[test]
fn loop_expression() {
    // loop pada rust merupakan perulangan tanpa batas. jadi butuh trigger break untuk menghentikan dan continue untuk melanjutkan ke iterasi berikutnya
    let mut counter: i32 = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break; // menghentikan perulangan
        } else if counter % 2 == 0 {
            continue; // melanjutkan ke iterasi berikutnya
        }
        println!("Counter: {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // menghentikan perulangan dan mengembalikan nilai counter * 2
        }
    };
    println!("Result: {}", result+1);
}

#[test]
fn loop_label() {
    let mut out = String::new();
    let mut number: i32 = 1;
    'outer: loop {
        let mut counter: i32 = 1;
        'inner: loop {
            if number > 100 {
                break 'outer; // menghentikan perulangan inner
            }

            out.push_str(&format!("{} x {} = {}\n", number, counter, number * counter));
            counter += 1;
            if counter > 100 {
                break 'inner
            }
        }
        number += 1;
    }
    println!("{}", out);
}

#[test]
fn loop_label_v2() {
    use std::fmt::Write; // biar bisa pakai write! ke String

    let mut out = String::new();
    let mut number: i32 = 1;
    'outer: loop {
        let mut counter: i32 = 1;
        'inner: loop {
            if number > 100 {
                break 'outer; // menghentikan perulangan inner
            }

            write!(&mut out, "{} x {} = {}\n", number, counter, number * counter).unwrap();

            counter += 1;
            if counter > 100 {
                break 'inner
            }
        }
        number += 1;
    }
    println!("{}", out);
}

#[test]
fn while_loop() {
    let mut counter: i32 = 1;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("{} is even", counter);
        }
        counter += 1;
    }
}