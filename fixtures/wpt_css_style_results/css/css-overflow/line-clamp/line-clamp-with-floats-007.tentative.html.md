# css/css-overflow/line-clamp/line-clamp-with-floats-007.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-with-floats-007.tentative.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: 4;
  font: 16px / 32px serif;
  padding: 0 4px;
  white-space: pre;
  background-color: yellow;
}
.float {
  float: left;
  margin: 4px;
  white-space: pre;
  background-color: skyblue;
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
