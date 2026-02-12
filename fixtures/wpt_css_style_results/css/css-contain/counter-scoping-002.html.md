# css/css-contain/counter-scoping-002.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/counter-scoping-002.html"
}
```

## style[0]

```css

div {
  contain: style;
  counter-set: n 1;
}
div::before, div::after {
  content: counters(n, '.') " ";
}
div::after {
  counter-set: n 2;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
