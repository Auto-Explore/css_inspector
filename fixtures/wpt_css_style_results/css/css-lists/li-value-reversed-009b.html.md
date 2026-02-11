# css/css-lists/li-value-reversed-009b.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/li-value-reversed-009b.html"
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
.pure-set {
  counter-increment: list-item 0;
  counter-set: list-item 5;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “counter-set”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
