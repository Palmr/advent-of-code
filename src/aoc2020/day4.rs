/// --- Day 4: Passport Processing ---
///
/// You arrive at the airport only to realize that you grabbed your North Pole Credentials instead
/// of your passport. While these documents are extremely similar, North Pole Credentials aren't
/// issued by a country and therefore aren't actually valid documentation for travel in most of the
/// world.
///
/// It seems like you're not the only one having problems, though; a very long line has formed for
/// the automatic passport scanners, and the delay could upset your travel itinerary.
///
/// Due to some questionable network security, you realize you might be able to solve both of these
/// problems at the same time.
///
/// The automatic passport scanners are slow because they're having trouble detecting which
/// passports have all required fields. The expected fields are as follows:
///
///     byr (Birth Year)
///     iyr (Issue Year)
///     eyr (Expiration Year)
///     hgt (Height)
///     hcl (Hair Color)
///     ecl (Eye Color)
///     pid (Passport ID)
///     cid (Country ID)
///
/// Passport data is validated in batch files (your puzzle input). Each passport is represented as
/// a sequence of key:value pairs separated by spaces or newlines. Passports are separated by blank
/// lines.
///
/// Here is an example batch file containing four passports:
///
/// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
/// byr:1937 iyr:2017 cid:147 hgt:183cm
///
/// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
/// hcl:#cfa07d byr:1929
///
/// hcl:#ae17e1 iyr:2013
/// eyr:2024
/// ecl:brn pid:760753108 byr:1931
/// hgt:179cm
///
/// hcl:#cfa07d eyr:2025 pid:166559648
/// iyr:2011 ecl:brn hgt:59in
///
/// The first passport is valid - all eight fields are present. The second passport is invalid - it
/// is missing hgt (the Height field).
///
/// The third passport is interesting; the only missing field is cid, so it looks like data from
/// North Pole Credentials, not a passport at all! Surely, nobody would mind if you made the system
/// temporarily ignore missing cid fields. Treat this "passport" as valid.
///
/// The fourth passport is missing two fields, cid and byr. Missing cid is fine, but missing any
/// other field is not, so this passport is invalid.
///
/// According to the above rules, your improved system would report 2 valid passports.
///
/// Count the number of valid passports - those that have all required fields. Treat cid as
/// optional. In your batch file, how many passports are valid?
///
/// --- Part Two ---
///
/// The line is moving more quickly now, but you overhear airport security talking about how
/// passports with invalid data are getting through. Better add some data validation, quick!
///
/// You can continue to ignore the cid field, but each other field has strict rules about what
/// values are valid for automatic validation:
///
///     byr (Birth Year) - four digits; at least 1920 and at most 2002.
///     iyr (Issue Year) - four digits; at least 2010 and at most 2020.
///     eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
///     hgt (Height) - a number followed by either cm or in:
///         If cm, the number must be at least 150 and at most 193.
///         If in, the number must be at least 59 and at most 76.
///     hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
///     ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
///     pid (Passport ID) - a nine-digit number, including leading zeroes.
///     cid (Country ID) - ignored, missing or not.
///
/// Your job is to count the passports where all required fields are both present and valid
/// according to the above rules. Here are some example values:
///
/// byr valid:   2002
/// byr invalid: 2003
///
/// hgt valid:   60in
/// hgt valid:   190cm
/// hgt invalid: 190in
/// hgt invalid: 190
///
/// hcl valid:   #123abc
/// hcl invalid: #123abz
/// hcl invalid: 123abc
///
/// ecl valid:   brn
/// ecl invalid: wat
///
/// pid valid:   000000001
/// pid invalid: 0123456789
///
/// Here are some invalid passports:
///
/// eyr:1972 cid:100
/// hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
///
/// iyr:2019
/// hcl:#602927 eyr:1967 hgt:170cm
/// ecl:grn pid:012533040 byr:1946
///
/// hcl:dab227 iyr:2012
/// ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
///
/// hgt:59cm ecl:zzz
/// eyr:2038 hcl:74454a iyr:2023
/// pid:3556412378 byr:2007
///
/// Here are some valid passports:
///
/// pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
/// hcl:#623a2f
///
/// eyr:2029 ecl:blu cid:129 byr:1989
/// iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
///
/// hcl:#888785
/// hgt:164cm byr:2001 iyr:2015 cid:88
/// pid:545766238 ecl:hzl
/// eyr:2022
///
/// iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
///
/// Count the number of valid passports - those that have all required fields and valid values.
/// Continue to treat cid as optional. In your batch file, how many passports are valid?
///
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_FIELD: Regex = Regex::new(r"^([a-z]+):(.+)$").unwrap();
    static ref RE_HGT: Regex = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
    static ref RE_PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref RE_HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
}

struct Passport {
    pub byr: Option<String>,
    pub iyr: Option<String>,
    pub eyr: Option<String>,
    pub hgt: Option<String>,
    pub hcl: Option<String>,
    pub ecl: Option<String>,
    pub pid: Option<String>,
    pub cid: Option<String>,
}

impl Passport {
    pub fn simple_validation(passport: &Passport) -> bool {
        passport.byr.is_some()
            && passport.iyr.is_some()
            && passport.eyr.is_some()
            && passport.hgt.is_some()
            && passport.hcl.is_some()
            && passport.ecl.is_some()
            && passport.pid.is_some()
        // cid is optional
    }

    pub fn fancy_validation(passport: &Passport) -> bool {
        passport.validate_byr()
            && passport.validate_iyr()
            && passport.validate_eyr()
            && passport.validate_hgt()
            && passport.validate_hcl()
            && passport.validate_ecl()
            && passport.validate_pid()
            && passport.validate_cid()
    }

    pub fn reset(&mut self) {
        self.byr = None;
        self.iyr = None;
        self.eyr = None;
        self.hgt = None;
        self.hcl = None;
        self.ecl = None;
        self.pid = None;
        self.cid = None;
    }

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    fn validate_byr(&self) -> bool {
        let byr = self
            .byr
            .as_ref()
            .map(|b| b.parse::<i32>().unwrap_or(-1))
            .unwrap_or(-1);

        (1920..=2002).contains(&byr)
    }

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    fn validate_iyr(&self) -> bool {
        let iyr = self
            .iyr
            .as_ref()
            .map(|b| b.parse::<i32>().unwrap_or(-1))
            .unwrap_or(-1);

        (2010..=2020).contains(&iyr)
    }

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    fn validate_eyr(&self) -> bool {
        let eyr = self
            .eyr
            .as_ref()
            .map(|b| b.parse::<i32>().unwrap_or(-1))
            .unwrap_or(-1);

        (2020..=2030).contains(&eyr)
    }

    // hgt (Height) - a number followed by either cm or in:
    fn validate_hgt(&self) -> bool {
        self.hgt
            .as_ref()
            .map(|hgt| {
                let captures = RE_HGT.captures(hgt.as_str());
                match captures {
                    None => false,
                    Some(captures) => {
                        let height: i32 = captures[1].parse().unwrap();
                        let unit = &captures[2];

                        //     If cm, the number must be at least 150 and at most 193.
                        //     If in, the number must be at least 59 and at most 76.
                        match unit {
                            "cm" => (150..=193).contains(&height),
                            "in" => (59..=76).contains(&height),
                            _ => panic!("Unrecognised height unit: {}", unit),
                        }
                    }
                }
            })
            .unwrap_or(false)
    }

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    fn validate_hcl(&self) -> bool {
        self.hcl
            .as_ref()
            .map(|hcl| RE_HCL.is_match(hcl.as_str()))
            .unwrap_or(false)
    }

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    fn validate_ecl(&self) -> bool {
        match self.ecl.as_deref().unwrap_or("") {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            &_ => false,
        }
    }

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    fn validate_pid(&self) -> bool {
        self.pid
            .as_ref()
            .map(|pid| RE_PID.is_match(pid.as_str()))
            .unwrap_or(false)
    }

    // cid (Country ID) - ignored, missing or not.
    fn validate_cid(&self) -> bool {
        true
    }
}

fn validate_passports<F>(input: &[String], validation_function: F) -> usize
where
    F: Fn(&Passport) -> bool,
{
    let mut passport_list: Vec<bool> = Vec::new();
    let mut passport = Passport {
        byr: None,
        iyr: None,
        eyr: None,
        hgt: None,
        hcl: None,
        ecl: None,
        pid: None,
        cid: None,
    };

    for line in input {
        if line.is_empty() {
            passport_list.push(validation_function(&passport));
            passport.reset();
        } else {
            line.split(' ').for_each(|field| {
                let field_parts = RE_FIELD.captures(field).unwrap();
                let field_name = &field_parts[1];
                let field_value = field_parts[2].to_owned();

                match field_name {
                    "byr" => passport.byr = Some(field_value),
                    "iyr" => passport.iyr = Some(field_value),
                    "eyr" => passport.eyr = Some(field_value),
                    "hgt" => passport.hgt = Some(field_value),
                    "hcl" => passport.hcl = Some(field_value),
                    "ecl" => passport.ecl = Some(field_value),
                    "pid" => passport.pid = Some(field_value),
                    "cid" => passport.cid = Some(field_value),
                    _ => {
                        panic!("Got an unrecognised field: {}", field)
                    }
                }
            });
        }
    }

    passport_list.push(validation_function(&passport));

    passport_list.iter().filter(|&p| p == &true).count()
}

pub fn solve_part_one(input: &[String]) -> usize {
    validate_passports(input, Passport::simple_validation)
}

pub fn solve_part_two(input: &[String]) -> usize {
    validate_passports(input, Passport::fancy_validation)
}

#[test]
fn examples_part_one() {
    assert_eq!(
        2,
        solve_part_one(&[
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
            "".to_string(),
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
            "hcl:#cfa07d byr:1929".to_string(),
            "".to_string(),
            "hcl:#ae17e1 iyr:2013".to_string(),
            "eyr:2024".to_string(),
            "ecl:brn pid:760753108 byr:1931".to_string(),
            "hgt:179cm".to_string(),
            "".to_string(),
            "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
            "iyr:2011 ecl:brn hgt:59in".to_string(),
        ])
    );
}

#[test]
fn examples_part_two() {
    assert_eq!(
        0,
        solve_part_two(&[
            "eyr:1972 cid:100".to_string(),
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(),
            "".to_string(),
            "iyr:2019".to_string(),
            "hcl:#602927 eyr:1967 hgt:170cm".to_string(),
            "ecl:grn pid:012533040 byr:1946".to_string(),
            "".to_string(),
            "hcl:dab227 iyr:2012".to_string(),
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(),
            "".to_string(),
            "hgt:59cm ecl:zzz".to_string(),
            "eyr:2038 hcl:74454a iyr:2023".to_string(),
            "pid:3556412378 byr:2007".to_string(),
        ])
    );

    assert_eq!(
        4,
        solve_part_two(&[
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980".to_string(),
            "hcl:#623a2f".to_string(),
            "".to_string(),
            "eyr:2029 ecl:blu cid:129 byr:1989".to_string(),
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".to_string(),
            "".to_string(),
            "hcl:#888785".to_string(),
            "hgt:164cm byr:2001 iyr:2015 cid:88".to_string(),
            "pid:545766238 ecl:hzl".to_string(),
            "eyr:2022".to_string(),
            "".to_string(),
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string(),
        ])
    );
}
