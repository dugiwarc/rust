fn gimme(input_array: [i32;3]) -> usize {
    let mut ret = input_array;
    ret.sort();
    input_array.iter().position(|&x| x == ret[1]).unwrap()
}