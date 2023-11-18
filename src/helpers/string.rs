
pub fn search_lines<'a>(content: &'a String, query: &String) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();

    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query))
        .map(|(index, line)| (index + 1, line.trim()))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_text() -> String {
        "\
        I'm nobody! Who are you?
        Are you nobody, too?
        Then there's a pair of us - don't tell!
        They'd banish us, you know.".to_string()
    }

    #[test]
    fn search_lines_no_result() {
        let haystack = test_text();
        let needle = "404";

        let search_results = search_lines(&haystack, &needle.to_string());

        assert_eq!(search_results.len(), 0, "Result length error");
    }

    #[test]
    fn search_lines_one_result() {
        let haystack = test_text();
        let needle = "don't TELL";

        let search_results = search_lines(&haystack, &needle.to_string());

        assert_eq!(search_results.len(), 1, "Result length error");
        assert_eq!(search_results[0].0, 3, "Result line error");
        assert_eq!(search_results[0].1, "Then there's a pair of us - don't tell!", "Result text error");
    }

    #[test]
    fn search_lines_three_result() {
        let haystack = test_text();
        let needle = "you";

        let search_results = search_lines(&haystack, &needle.to_string());

        assert_eq!(search_results.len(), 3, "Result length error");

        assert_eq!(search_results[0].0, 1, "First result line error");
        assert_eq!(search_results[0].1, "I'm nobody! Who are you?", "First result text error");

        assert_eq!(search_results[1].0, 2, "Second result line error");
        assert_eq!(search_results[1].1, "Are you nobody, too?", "Second result text error");

        assert_eq!(search_results[2].0, 4, "Third result line error");
        assert_eq!(search_results[2].1, "They'd banish us, you know.", "Third result text error");
    }
    

}
