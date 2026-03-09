//! Shared constants and regex patterns used by both [`frontmatter`] and
//! [`agent_frontmatter`] scanners.
//!
//! Extracted here to avoid duplicating identical definitions across the two
//! scanner modules while keeping each scanner self-contained otherwise.

use std::sync::LazyLock;

/// Generic terms that by themselves make a skill/agent name meaningless.
pub const VAGUE_NAME_TERMS: &[&str] = &["helper", "utils", "tools", "data", "files", "documents"];

/// First/second-person patterns that indicate a description is not written in
/// third person, as required by the Claude agent skills best-practices guide.
pub static RE_FIRST_PERSON: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?i)\b(I can|I will|I'll|I am|I'm|you can|you should|you will|you'll)\b")
        .unwrap()
});

/// Windows-style backslash path separator. Forward slashes should be used in
/// SKILL.md / AGENT.md to ensure cross-platform compatibility.
pub static RE_WINDOWS_PATH: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[a-zA-Z]:\\|[a-zA-Z0-9_][\\][a-zA-Z0-9_]").unwrap());

/// Date-conditional language that will become stale over time.
/// Matches patterns like "before August 2025", "after 2024", "as of January 2026".
pub static RE_TIME_SENSITIVE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)\b(before|after|until|since|as of|by)\s+\w*\s*(january|february|march|april|may|june|july|august|september|october|november|december)?\s*\d{4}\b",
    )
    .unwrap()
});

/// Keyword phrases that signal "when to use" context in a description.
pub const TRIGGER_PHRASES: &[&str] = &[
    "use when",
    "when the user",
    "when working with",
    "when asked",
    "when you need",
    "trigger",
    "invoke when",
    "should be used when",
    "when users",
    "when a user",
    "use it when",
    "useful when",
];
