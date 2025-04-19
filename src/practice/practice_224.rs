use core::panic;

#[derive(Debug)]
enum Op {
    Operand(i32),
    Operator(char),
}
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let (ans, _) = recur(s.as_bytes(), 0);
        ans
    }
}

fn recur(s: &[u8], mut idx: usize) -> (i32, usize) {
    let mut stack = Vec::new();
    while idx < s.len() {
        match s[idx] as char {
            '+' | '-' => {
                stack.push(Op::Operator(s[idx] as char));
                idx += 1;
            }
            '(' => {
                let (result, new_idx) = recur(s, idx + 1);
                stack.push(Op::Operand(result));
                idx = new_idx;
            }
            ' ' => idx += 1,
            ')' => return (calc_stack(&mut stack), idx + 1),
            _ => {
                let (operand, new_idx) = parse_operand(s, idx);
                stack.push(Op::Operand(operand));
                idx = new_idx;
            }
        }
    }
    (calc_stack(&mut stack), s.len())
}

fn calc_stack(stack: &mut Vec<Op>) -> i32 {
    if stack.len() == 0 {
        return 0;
    }

    let mut ans = 0;
    let mut idx = 0;

    if let Op::Operand(first_operand) = stack[0] {
        ans = first_operand;
        idx += 1;
    }

    while idx < stack.len() {
        let mid_op = &stack[idx];
        let right_op = &stack[idx + 1];
        dbg!(mid_op, right_op);
        idx += 2;
        if let (&Op::Operand(right_operand), &Op::Operator(op)) = (right_op, mid_op) {
            ans = match op {
                '+' => ans + right_operand,
                _ => ans - right_operand,
            }
        } else {
            panic!("should not happened")
        }
    }

    ans
}

fn parse_operand(chars: &[u8], idx: usize) -> (i32, usize) {
    let mut num = 0i32;
    let mut last_idx = 0;
    for i in idx..chars.len() {
        if chars[i] >= b'0' && chars[i] <= b'9' {
            num = num * 10 + (chars[i] - b'0') as i32;
            last_idx = i;
        } else {
            break;
        }
    }
    (num, last_idx + 1)
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(Solution::calculate(String::from("(1+(4+5+2)-3)+(6+8)")), 23);
        assert_eq!(
            Solution::calculate(String::from("(1-(4+5-20)+3)-(-2+6+8)")),
            3
        );

        assert_eq!(Solution::calculate(String::from("-2+1")), -1);
        assert_eq!(Solution::calculate(String::from("(2-1)+2")), 3);
    }
}
