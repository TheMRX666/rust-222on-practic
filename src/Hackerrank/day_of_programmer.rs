fn dayOfProgrammer(year: i32) -> String {

    let days__months_normal = 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31;
    let days__months_leap = 31 + 29 + 31 + 30 + 31 + 30 + 31 + 31;

    let (day, month) = match year {
        1700..=1917 if year % 4 == 0 =>
            (256 - (days__months_leap), 9),

        1700..=1917 =>
            (256 - (days__months_normal), 9),

        1918 =>
            (256 - (31 + 15 + 31 + 30 + 31 + 30 + 31 + 31), 9),

        1919..=2700 if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) =>
            (256 - (days__months_leap), 9),

        1919..=2700 =>
            (256 - (days__months_normal), 9),

        _ => unreachable!(),
    };

    format!("{:02}.{:02}.{}", day, month, year)
}

#[test]
fn main() {
    let year = 2023;
    let expected = "13.09.2023";
    let result = dayOfProgrammer(year);
    assert_eq!(result, expected, "For year {}, expected {}, but got {}", year, expected, result);
}
