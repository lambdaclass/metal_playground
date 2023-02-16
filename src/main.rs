use metal_playground::dotprod;

fn main() {
    let result = dotprod::dot(&[3, 4, 1, 7], &[2, 5, 6, 9]);
    unsafe { println!("{:?}", *result) }
}
