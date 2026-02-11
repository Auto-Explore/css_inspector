# css/css-lists/li-value-reversed-006-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/li-value-reversed-006-ref.html"
}
```

## style[0]

```css

:root {
  color:black; background-color:white; font:10px/1 monospace;
}
ol::before {
  content: counters(list-item,".");
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
