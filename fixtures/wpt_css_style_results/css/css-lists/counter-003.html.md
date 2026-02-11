# css/css-lists/counter-003.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counter-003.html"
}
```

## style[0]

```css

#test { counter-reset: c; }
#test span { counter-increment: c; }
#test span::before { content: counter(c, upper-roman); }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
