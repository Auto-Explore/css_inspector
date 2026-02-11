# css/css-overflow/line-clamp/line-clamp-with-abspos-005.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-with-abspos-005.html"
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
  position: relative;
}
.abspos {
  position: absolute;
  top: 0;
  left: 0;
  width: 20px;
  height: 20px;
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
