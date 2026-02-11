# css/css-overflow/line-clamp/block-ellipsis-repaint-001.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/block-ellipsis-repaint-001.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: 1;
  border: 1px solid black;
  padding: 4px;
  background-color: yellow;
}
#atomic {
  display: inline-block;
  background-color: orange;
  width: 2em;
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
