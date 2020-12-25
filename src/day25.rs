use aoc_runner_derive::aoc;

const CARD_PK: u64 = 16915772;
const DOOR_PK: u64 = 18447943;
const SUBJECT: u64 = 7;

fn transform(subject_number: u64) -> impl Iterator<Item = u64> {
    (0..).scan(1, move |res, _| {
        *res = (*res * subject_number) % 20201227;
        Some(*res)
    })
}

fn find_loop_size(pk: u64) -> usize {
    transform(SUBJECT).position(|x| x == pk).expect("Valid PK!")
}

fn encryption_key(pk1: u64, pk2: u64) -> u64 {
    let pk1_loop_size = find_loop_size(pk1);
    transform(pk2).nth(pk1_loop_size).unwrap()
}

#[aoc(day25, part1)]
pub fn part1(_: &str) -> u64 {
    encryption_key(CARD_PK, DOOR_PK)
}

#[cfg(test)]
mod test_day25 {
    use super::*;

    const CARD_PK: u64 = 5764801;
    const DOOR_PK: u64 = 17807724;

    #[test]
    fn test_part1() {
        assert_eq!(find_loop_size(CARD_PK), 8 - 1);
        assert_eq!(find_loop_size(DOOR_PK), 11 - 1);
        let key1 =  encryption_key(CARD_PK, DOOR_PK);
        let key2 = encryption_key(DOOR_PK, CARD_PK);
        assert_eq!(key1, key2);
        assert_eq!(key1, 14897079)
    }

}
