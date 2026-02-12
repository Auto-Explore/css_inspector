# css/css-lists/li-value-reversed-007b.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/li-value-reversed-007b.html"
}
```

## style[0]

```css

:root {
  color:black; background-color:white; font:10px/1 monospace;
}
ol::before, li::before {
  content: counters(list-item,".");
}
div, span {
  counter-increment: list-item 0;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
