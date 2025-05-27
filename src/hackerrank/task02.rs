fn compare_triplets(a: [i32; 3], b: [i32; 3]) -> [i32; 2] {
    let mut alice_score = 0;
    let mut bob_score = 0;

    for i in 0..3 {
        if a[i] > b[i] {
            alice_score += 1;
        } else if a[i] < b[i] {
            bob_score += 1;
        }
    }

    [alice_score, bob_score]
}
