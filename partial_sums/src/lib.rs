pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    let mut result: Vec<u64> = Vec::new();
    let mut sum: u64 = 0;

    for ele in arr {
        println!("{:?}", ele);
        sum += ele;
        res.push(sum);
    }
    for ele in res.iter().rev() {
        result.push(*ele);
    }
    result.push(0);
    result
}
