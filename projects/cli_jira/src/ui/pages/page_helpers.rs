use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    if width <= 3 {
        return ".".repeat(width);
    }

    if width >= text.len() {
        let whitespace = " ".repeat(width - text.len());

        return format!("{text}{whitespace}");
    }

    text.truncate_ellipse(width - 3).to_string()
}
