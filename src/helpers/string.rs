/// Find each line of content content containing query
/// 
/// # Arguments
///
/// * `content` - A String reference that contains the content to search in
/// * `query` - A String reference that contains the search term
/// 
/// 
/// # Examples
/// 
/// ```
/// use oms::helpers::string;
/// let content = "\
///        I'm nobody! Who are you?
///        Are you nobody, too?
///        Then there's a pair of us - don't tell!
///        They'd banish us, you know.".to_string();
/// let search_term = "you".to_string();
/// 
/// let mut lines = string::search_lines(&content, &search_term);
/// 
/// assert_eq!(Some((1 as usize, "I'm nobody! Who are you?")), lines.next());
/// assert_eq!(Some((2 as usize, "Are you nobody, too?")), lines.next());
/// assert_eq!(Some((4 as usize, "They'd banish us, you know.")), lines.next());
/// ```
pub fn search_lines<'a>(content: &'a String, query: &'a str) -> impl Iterator<Item = (usize, &'a str)> {
    let query = query.to_lowercase();

    content
        .lines()
        .enumerate()
        .filter(move |(_, line)| line.to_lowercase().contains(&query))
        .map(|(index, line)| (index + 1, line.trim()))
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
        let content = test_text();
        let mut result = search_lines(&content, "404");
        assert_eq!(None, result.next());
    }

    #[test]
    fn search_lines_one_result() {
        let content = test_text();
        let mut result = search_lines(&content, "don't TELL");
        assert_eq!(Some((3 as usize, "Then there's a pair of us - don't tell!")), result.next());
        assert_eq!(None, result.next());
    }

    #[test]
    fn search_lines_three_result() {
        let content = test_text();
        let mut iter = search_lines(&content, "you");
        assert_eq!(Some((1 as usize, "I'm nobody! Who are you?")), iter.next());
        assert_eq!(Some((2 as usize, "Are you nobody, too?")), iter.next());
        assert_eq!(Some((4 as usize, "They'd banish us, you know.")), iter.next());
        assert_eq!(None, iter.next());
    }
}