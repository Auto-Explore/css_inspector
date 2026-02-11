# css/css-overflow/line-clamp/line-clamp-with-floats-002.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-with-floats-002.tentative.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: 4;
  font: 16px / 32px serif;
  padding: 0 4px;
  background-color: yellow;
}
.float {
  float: left;
  width: 50px;
  height: 50px;
  margin: 4px;
  background-color: skyblue;
}
.pre {
  white-space: pre;
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
