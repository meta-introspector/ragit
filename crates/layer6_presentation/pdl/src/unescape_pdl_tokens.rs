pub fn unescape_pdl_tokens(s: &str) -> String {
    // TODO: use `Cow` type
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}
