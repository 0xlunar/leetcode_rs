impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(tokens.len());

        for token in tokens {
            match token.as_str() {
                "+" => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();
                    stack.push(right + left);
                }
                "-" => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();
                    stack.push(right - left);
                }
                "*" => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();
                    stack.push(right * left);
                }
                "/" => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();
                    stack.push(right / left);
                }
                _ => stack.push(token.parse::<i32>().unwrap()),
            }
        }

        stack.first().unwrap().to_owned()
    }
}
