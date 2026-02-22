use ratatui::{backend::TestBackend, Terminal};

pub fn create_test_terminal() -> Terminal<TestBackend> {
    let backend = TestBackend::new(80, 24);
    Terminal::new(backend).unwrap()
}

pub fn create_test_terminal_with_size(width: u16, height: u16) -> Terminal<TestBackend> {
    let backend = TestBackend::new(width, height);
    Terminal::new(backend).unwrap()
}

pub fn extract_buffer_content(terminal: &Terminal<TestBackend>) -> String {
    let backend = terminal.backend();
    let buffer = backend.buffer();
    buffer
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>()
}

pub fn buffer_contains_text(terminal: &Terminal<TestBackend>, text: &str) -> bool {
    extract_buffer_content(terminal).contains(text)
}

pub fn count_pattern_in_buffer(terminal: &Terminal<TestBackend>, pattern: &str) -> usize {
    extract_buffer_content(terminal).matches(pattern).count()
}
