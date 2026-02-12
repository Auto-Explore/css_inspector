# css/css-lists/li-value-reversed-006b.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/li-value-reversed-006b.html"
}
```

## style[0]

```css

:root {
  color:black; background-color:white; font:10px/1 monospace;
}
ol {
  counter-reset: reversed(list-item);
}
ol::before, li::before {
  content: counters(list-item,".");
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
