#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_part1() {
        let sample = "";

        assert_eq!("", part1(sample));
    }
}
