# css/css-contain/reference/contain-style-counters-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/reference/contain-style-counters-005-ref.html"
}
```

## style[0]

```css

.contain {
    float: right;
    clear: both;
}
.reset { counter-reset: c;}
.increment:before { content: counters(c, ""); }
.increment { counter-increment: c; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
