# spanda-lexer

Spanda source tokenizer: `Token`, `TokenType`, `tokenize()`, unit suffix lexemes, and `reserved_keywords()`.

Phase 4 compiler split crate — depends on `spanda-ast` for `UnitKind` mapping. Returns `LexerError` diagnostics; `spanda-core` maps them to `SpandaError`.
