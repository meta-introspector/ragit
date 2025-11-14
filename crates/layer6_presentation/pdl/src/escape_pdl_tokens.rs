pub fn escape_pdl_tokens(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("|>", "|&gt;")
        .replace("<|", "&lt;|")
}
