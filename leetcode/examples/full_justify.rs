struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut line: Vec<String> = Vec::new();
        let mut current_width = 0;

        for word in words {
            let word_width = word.len();
            let line_len = line.len();

            if current_width + word_width + line_len <= max_width as usize {
                line.push(word);
                current_width += word_width;
            } else {
                let num_words = line.len();
                let num_spaces = max_width as usize - current_width;

                let mut line_str = String::new();

                if num_words == 1 {
                    line_str += &line[0];
                    line_str += &" ".repeat(num_spaces);
                } else {
                    let spaces_between_words = num_spaces / (num_words - 1);
                    let extra_spaces = num_spaces % (num_words - 1);

                    for i in 0..num_words {
                        line_str += &line[i];

                        if i < num_words - 1 {
                            let spaces = spaces_between_words + if i < extra_spaces { 1 } else { 0 };
                            line_str += &" ".repeat(spaces);
                        }
                    }
                }

                result.push(line_str);

                line.clear();
                line.push(word);
                current_width = word_width;
            }
        }

        let last_line = line.join(" ");
        let last_line_width = last_line.len();
        let padding = max_width as usize - last_line_width;
        let padding_str = " ".repeat(padding);

        result.push(last_line + &padding_str);

        result
    }
}


fn main () {
    
}