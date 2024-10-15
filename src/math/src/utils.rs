/**
* Range sum returns the sum of the numbers between low and high
*/
fn range_sum(low: i32, high: i32) -> i32 {
    let mut low = low;
    let mut total = 0;

    while low <= high {
        total += low;
        low += 1;
    }

    total
}
