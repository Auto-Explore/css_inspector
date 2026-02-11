# css/css-lists/counter-001.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counter-001.html"
}
```

## style[0]

```css

#test { counter-reset: c; }
#test span { counter-increment: c; }
#test span::before { content: counter(c, decimal-leading-zero); }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
