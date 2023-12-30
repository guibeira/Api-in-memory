pub fn check_number(number: usize) -> String {
    let mut output = String::new();
    output.push_str(&("Type 1".repeat((number % 3 == 0) as usize)));
    output.push_str(&("Type 2".repeat((number % 5 == 0) as usize)));

    if output.is_empty() {
        return format!("{}", number);
    } else {
        return format!("{}", output.replace("Type 1Type 2", "Type 3"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_multiple_of_three() {
        let numbers = vec![3, 6, 9, 12, 18, 21, 24, 27, 33, 36, 39];
        for n in numbers {
            assert_eq!(check_number(n), "Type 1");
        }
    }

    #[test]
    fn check_multiple_of_five() {
        let numbers = vec![5, 10, 20, 25, 35, 40, 50, 55, 65, 70, 80];
        for n in numbers {
            assert_eq!(check_number(n), "Type 2");
        }
    }

    #[test]
    fn check_multiple_of_three_and_five() {
        let numbers = vec![15, 30, 45, 60, 75, 90, 105, 120, 135, 150];
        for n in numbers {
            assert_eq!(check_number(n), "Type 3");
        }
    }

    #[test]
    fn check_not_multiple_of_three_or_five() {
        let numbers = vec![1, 2, 4, 7, 8, 11, 13, 14, 16, 17, 19];
        for n in numbers {
            assert_eq!(check_number(n), format!("{}", n));
        }
    }
}
