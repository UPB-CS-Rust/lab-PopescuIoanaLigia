fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // let mut max = 0;
    // let mut min = 99;
    // for i in input {
    //     if i> max{
    //         max = i;
    //     }
    //     if i< min{
    //         min = i;
    //     }
    // }


    let min = input.iter().min().unwrap_or(&0);
    let max = input.iter().max().unwrap_or(&0);

    println!("{:?} is largest and {:?} is smallest", max, min);
}
