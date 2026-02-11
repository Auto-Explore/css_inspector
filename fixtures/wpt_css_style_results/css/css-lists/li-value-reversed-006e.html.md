# css/css-lists/li-value-reversed-006e.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/li-value-reversed-006e.html"
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
ol[reversed] {
  margin-bottom: 0;
}
ol.no-counter {
  counter-reset: blah;
  margin-top: 0;
}
ol.no-counter::before {
  content: none;
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
