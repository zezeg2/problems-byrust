use crate::Solution;

struct ZigzagString {
    origin: String,
    row: usize,
}
impl ZigzagString {
    pub fn new(origin: String, row: usize) -> Self {
        ZigzagString { origin, row }
    }

    pub fn convert(&self) -> String {
        let mut vv: Vec<Vec<char>> = vec![vec![]; self.row as usize];
        let cycle: usize = self.row * 2 - 2;

        if cycle == 0 {
            return self.origin.clone();
        }

        for (u, c) in self.origin.chars().enumerate() {
            let nn = u % cycle;
            let tg_row: usize;
            match nn / self.row {
                0 => tg_row = nn % self.row,
                1 => tg_row = self.row - 1 - (nn % self.row + 1),
                _ => {
                    panic!("Not Possible")
                }
            }
            vv[tg_row].push(c);
        }

        vv.iter()
            .map(|v| v.iter().collect::<String>())
            .collect::<String>()
    }
}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        ZigzagString::new(s, num_rows as usize).convert()
    }
}


#[cfg(test)]
mod test {
    use crate::Solution;
    #[test]
    fn test_case() {
        let result = Solution::convert("PAYPALISHIRING".to_string(), 4);
        assert_eq!(result, "PINALSIGYAHRPI".to_string())
    }
}
