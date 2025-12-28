enum TagType {
    ForTag,
    IfTag,
}

enum ContentType {
    Literal(String),
    Variable,
    Tag(),
    Unrecognized,
}
