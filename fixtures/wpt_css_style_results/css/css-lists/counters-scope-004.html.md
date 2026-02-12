# css/css-lists/counters-scope-004.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counters-scope-004.html"
}
```

## style[0]

```css

.reset { counter-reset: c; }
.rb::before { counter-reset: c; content: "R"; }
.use { counter-increment: c; }
.use::before { content: counters(c, ".") " "; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
