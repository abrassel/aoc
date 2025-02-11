pub fn cost(x: i32) -> i32 {
    x / 3 - 2
}

fn part_1(input: &[i32]) -> i32 {
    input.iter().map(|x| cost(*x)).sum()
}

fn compute_total(x: i32) -> i32 {
    let mut tot = 0;
    let mut cur = cost(x);
    while cur > 0 {
        tot += cur;
        cur = cost(cur);
    }
    tot
}

fn part_2(input: &[i32]) -> i32 {
    input.iter().map(|x| compute_total(*x)).sum()
}

pub fn run() {
    let input: Vec<i32> = crate::utls::read_text_from_file("2019", "01");
    part_1(&input);
    let sol = part_2(&input);
    println!("solution is {}", sol);
}
