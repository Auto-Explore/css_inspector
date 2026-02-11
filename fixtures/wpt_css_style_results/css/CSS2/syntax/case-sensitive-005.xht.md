# css/CSS2/syntax/case-sensitive-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/case-sensitive-005.xht"
}
```

## style[0]

```css

    .reset { counter-reset: test 5 tEsT 9; }
    .incr  { counter-reset: TEST 0 tEsT 0; counter-increment: TEST 5 tEsT 9; }
    span:before { content: counter(tEsT); }
    span { color: blue; font: larger bold monospace; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
