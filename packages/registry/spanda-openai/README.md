# spanda-openai

Official Spanda package: **OpenAI LLM via Python bridge**

## Import

```spanda
import ai.openai;
```

## Live backend

`src/ai_openai.sd` declares `extern python` bindings. Runtime AI hooks remain in
`spanda-core/src/ai.rs` until an `AiProvider` registration path lands in package bootstrap.

## Status

Partial implementation — extern Python surface exists; full provider trait wiring is planned
for Phase 5 package backends.
