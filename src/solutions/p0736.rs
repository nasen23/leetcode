use std::collections::{HashMap, VecDeque};
use std::{slice, str};

struct SymbolTable {
    var_maps: VecDeque<HashMap<String, i32>>,
}

impl SymbolTable {
    fn new() -> Self {
        let mut st = VecDeque::new();
        st.push_back(HashMap::new());
        Self { var_maps: st }
    }

    fn get(&self, key: &String) -> Option<i32> {
        self.var_maps.back().unwrap().get(key).copied()
    }

    fn insert(&mut self, key: String, value: i32) {
        self.var_maps.back_mut().unwrap().insert(key, value);
    }

    fn push(&mut self) {
        self.var_maps
            .push_back(self.var_maps.back().unwrap().clone());
    }

    fn pop(&mut self) {
        self.var_maps.pop_back();
    }
}

struct Parser<'a> {
    src: &'a str,
    ptr: *const u8,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            src: input,
            ptr: input.as_ptr(),
            pos: 0,
        }
    }

    #[inline]
    fn peek(&self) -> u8 {
        // debug_assert!(self.pos <= self.src.len());

        unsafe { *self.ptr.offset(self.pos as isize) }
    }

    #[inline]
    fn next(&mut self) {
        self.pos += 1;
    }

    #[inline]
    fn move_back(&mut self, n: usize) {
        self.pos -= n;
    }

    fn read_while<P>(&mut self, predicate: P) -> &'a str
    where
        P: Fn(u8) -> bool,
    {
        let start = self.pos;
        while predicate(self.peek()) {
            self.next();
        }
        unsafe {
            let ptr = self.ptr.offset(start as isize);
            let len = self.pos - start;
            str::from_utf8_unchecked(slice::from_raw_parts(ptr, len))
        }
    }

    fn skip_while<P>(&mut self, predicate: P)
    where
        P: Fn(u8) -> bool,
    {
        while predicate(self.peek()) {
            self.next();
        }
    }

    fn read_number(&mut self) -> i32 {
        let res = self.read_while(|x| x == b'-' || x.is_ascii_digit());
        i32::from_str_radix(res, 10).unwrap()
    }

    fn read_word(&mut self) -> &'a str {
        self.read_while(|x| !x.is_ascii_whitespace())
    }

    fn skip_whitespace(&mut self) {
        self.skip_while(|x| x.is_ascii_whitespace())
    }
}

struct LispEval<'a> {
    parser: Parser<'a>,
    vars: SymbolTable,
}

impl<'a> LispEval<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            parser: Parser::new(input),
            vars: SymbolTable::new(),
        }
    }

    fn eval(&mut self) -> i32 {
        match self.parser.peek() {
            b'-' | b'0'..=b'9' => self.parser.read_number(),
            b'(' => {
                self.parser.next();
                let op = self.parser.read_word();
                self.parser.skip_whitespace();

                let res = match op {
                    "add" => self.eval_add(),
                    "mult" => self.eval_mult(),
                    "let" => self.eval_let(),
                    _ => panic!("error"),
                };
                self.parser.skip_whitespace();
                self.parser.next();
                res
            }
            _ => {
                let name = self
                    .parser
                    .read_while(|x| !x.is_ascii_whitespace() && x != b')');
                self.vars.get(&name.to_string()).unwrap_or_else(|| {
                    println!("{}", name);
                    panic!("here");
                })
            }
        }
    }

    fn eval_add(&mut self) -> i32 {
        let left = self.eval();
        self.parser.skip_whitespace();
        let right = self.eval();
        left + right
    }

    fn eval_mult(&mut self) -> i32 {
        let left = self.eval();
        self.parser.skip_whitespace();
        let right = self.eval();
        left * right
    }

    fn eval_let(&mut self) -> i32 {
        self.vars.push();
        let res = loop {
            let name = self
                .parser
                .read_while(|x| !x.is_ascii_whitespace() && x != b')');
            if self.parser.peek() == b')'
                || name.starts_with('(')
                || name.starts_with(|ch: char| ch.is_ascii_digit())
            {
                println!("{}", name);
                self.parser.move_back(name.as_bytes().len());
                break self.eval();
            }
            self.parser.skip_whitespace();
            let value = self.eval();
            self.vars.insert(name.into(), value);
            self.parser.skip_whitespace();
        };
        self.vars.pop();
        res
    }
}

impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        let mut evaluator = LispEval::new(&expression);
        evaluator.eval()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn parse_lisp_expression() {
        assert_eq!(Solution::evaluate("(add 1 2)".into()), 3);
        assert_eq!(Solution::evaluate("(mult 3 (add 2 3))".into()), 15);
        assert_eq!(Solution::evaluate("(let x 2 (mult x 5))".into()), 10);
        assert_eq!(
            Solution::evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))".into()),
            14
        );
        assert_eq!(Solution::evaluate("(let x 3 x 2 x)".into()), 2);
        assert_eq!(
            Solution::evaluate("(let x 1 y 2 x (add x y) (add x y))".into()),
            5
        );
        assert_eq!(
            Solution::evaluate("(let x 2 (add (let x 3 (let x 4 x)) x))".into()),
            6
        );
        assert_eq!(Solution::evaluate("(let a1 3 b2 (add a1 1) b2) ".into()), 4);
        assert_eq!(Solution::evaluate("(let x 7 -12)".into()), -12);
    }
}
