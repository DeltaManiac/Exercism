pub fn sum_of_squares(count: i64) -> i64 {
    let mut sum: i64 = 0;
    for x in 1..count + 1 {
        sum = sum + x.pow(2);
    }
    sum
}

pub fn square_of_sum(count: i64) -> i64 {
    let mut sum: i64 = 0;
    for x in 1..count + 1 {
        sum = sum + x;
    }
    sum.pow(2)
}

pub fn difference(count: i64) -> i64 {
    square_of_sum(count) - sum_of_squares(count)
}
