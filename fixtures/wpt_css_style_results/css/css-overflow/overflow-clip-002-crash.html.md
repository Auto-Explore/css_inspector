# css/css-overflow/overflow-clip-002-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-002-crash.html"
}
```

## style[0]

```css

.item {
  background: cyan;
  background-attachment: local;
  overflow: clip;
  border-style: solid
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
