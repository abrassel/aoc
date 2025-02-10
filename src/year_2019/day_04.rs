fn part_1(input: &'static str) {}

fn part_2(input: &'static str) {}

pub fn run() {
    let input = crate::utls::read_text_from_file("2019", "04").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_solution() {}
}