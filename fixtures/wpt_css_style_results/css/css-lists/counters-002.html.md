# css/css-lists/counters-002.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counters-002.html"
}
```

## style[0]

```css

body, #test span:first-child { counter-reset: c; }
p, #test span { counter-increment: c; }
#test span::before { content: counters(c, ".", lower-roman); }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
