# css/css-lists/counters-001.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counters-001.html"
}
```

## style[0]

```css

body, #test span:first-child { counter-reset: c; }
p, #test span { counter-increment: c; }
#test span::before { content: counters(c, ".", decimal-leading-zero); }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
