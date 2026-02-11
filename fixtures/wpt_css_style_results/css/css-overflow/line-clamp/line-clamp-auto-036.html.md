# css/css-overflow/line-clamp/line-clamp-auto-036.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-auto-036.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: auto;
  max-height: 7lh;
  font: 16px / 32px serif;
  white-space: pre;
  padding: 0 4px;
  background-color: yellow;
}
.with-height {
  height: 3lh;
  background-color: orange;
}
.red {
  background-color: red;
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
