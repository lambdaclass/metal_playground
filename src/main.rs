use metal_playground::dotprod;

fn main() {
    let result = dotprod::dot([3_u32, 4, 1, 7], [2_u32, 5, 6, 9]);
    unsafe { println!("{:?}", *result) }
}
