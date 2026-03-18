fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let _zeros = [0; 10]; // [0, 0, 0, ..., 0] (10 elements)
    let _first = arr[0];
    let _len = arr.len();
    let slice: &[i32] = &arr[1..3]; // borrow a slice

    println!("{:?}", slice);
}
