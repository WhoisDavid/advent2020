use aoc_runner_derive::aoc;
use recap::Recap;
use serde::Deserialize;

/* Part 1 */

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?x)(
    (
        (byr: (?P<byr> [^\s]+ ))|
        (iyr: (?P<iyr> [^\s]+ ))|
        (eyr: (?P<eyr> [^\s]+ ))|
        (hgt: (?P<hgt> [^\s]+ ))|
        (hcl: (?P<hcl> [^\s]+ ))|
        (ecl: (?P<ecl> [^\s]+ ))|
        (pid: (?P<pid> [^\s]+ ))|
        (cid: (?P<cid> [^\s]+ ))
    )[\s\n]*
)+")]
pub struct PassportPart1 {
    byr: String,         // byr (Birth Year)
    iyr: String,         // iyr (Issue Year)
    eyr: String,         // eyr (Expiration Year)
    hgt: String,         // hgt (Height)
    hcl: String,         // hcl (Hair Color)
    ecl: String,         // ecl (Eye Color)
    pid: String,         // pid (Passport ID)
    cid: Option<String>, // cid (Country ID) - Optional
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(|s| s.parse::<PassportPart1>().ok()) // Passport is valid if it parses
        .count()
}

/* Part 2 */
#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?x)(
    (
        (byr:(?P<byr> \d{4} ))|
        (iyr:(?P<iyr> \d{4} ))|
        (eyr:(?P<eyr> \d{4} ))|
        (hgt:(?P<hgt> \d{2,3} )(?P<hgt_unit> (cm|in) ) )|
        (hcl:(?P<hcl> \#[0-9a-f]{6} ))|
        (ecl:(?P<ecl> (amb|blu|brn|gry|grn|hzl|oth) ))|
        (pid:(?P<pid> \d{9} ))|
        (cid:(?P<cid> [^\s]+ ))
    )[\s\n]*
)+")]
pub struct PassportPart2 {
    byr: u32, // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr: u32, // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr: u32, // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt: u8,  // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193. If in, the number must be at least 59 and at most 76.
    hgt_unit: String,    // cm or in
    hcl: String,         // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl: String,         // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid: String,         // pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid: Option<String>, // cid (Country ID) - ignored, missing or not.
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(|s| s.parse::<PassportPart2>().ok())
        .filter(|p| {
            (1920..=2002).contains(&p.byr)
                && (2010..=2020).contains(&p.iyr)
                && (2020..=2030).contains(&p.eyr)
                && (p.hgt_unit == "cm" && (150..=193).contains(&p.hgt)
                    || p.hgt_unit == "in" && (59..=76).contains(&p.hgt))
        })
        .count()
}

#[cfg(test)]
mod test_day4 {
    use super::*;

    static TESTCASE: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    static VALID_PASSPORTS: &str = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";

    static INVALID_PASSPORTS: &str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTCASE), 2)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 2);
        assert_eq!(part2(&input_parser(VALID_PASSPORTS)), 4);
        assert_eq!(part2(&input_parser(INVALID_PASSPORTS)), 0);
    }
}
