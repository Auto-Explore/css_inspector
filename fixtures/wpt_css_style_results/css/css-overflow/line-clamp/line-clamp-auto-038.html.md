# css/css-overflow/line-clamp/line-clamp-auto-038.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-auto-038.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: auto;
  max-height: 4lh;
  font: 16px / 32px serif;
  white-space: pre;
  background-color: yellow;
  border: 1px solid black;
  padding: 4px;
}
#inline-block {
  display: inline-block;
  background-color: orange;
  height: 0.5lh;
  width: 1em;
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
