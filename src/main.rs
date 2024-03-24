use lambdaworks_math::field::{traits::IsField, fields::u64_goldilocks_field};

fn getmoduloinv () -> u64{
    let val  = u64_goldilocks_field::Goldilocks64Field::inv(&2).unwrap();
    return val;

}

fn main() {
    let modulo = getmoduloinv();
    println!("The modulo inverse is: {:?}", modulo);
}

// The inverse is: 9223372034707292161