# css/css-overflow/line-clamp/line-clamp-auto-015.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-auto-015.tentative.html"
}
```

## style[0]

```css

.parent {
  padding: 0 4px;
  background-color: yellow;
}
.clamp {
  line-clamp: auto;
  max-height: 5lh;
  font: 16px / 32px serif;
  white-space: pre-wrap;
  width: 200px;
}
.float {
  float: left;
  width: 300px;
  background-color: orange;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “line-clamp”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
