# css/css-contain/contain-style-counters-004.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-style-counters-004.html"
}
```

## style[0]

```css

body {
  counter-reset: counter-of-span 13;
  contain: style;
}
body span {
  counter-increment: counter-of-span 5;
}
div {
  font-size: 3em;
}
div::after {
  content: counter(counter-of-span);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
