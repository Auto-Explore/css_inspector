# css/css-masking/animations/mask-border-source-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/animations/mask-border-source-interpolation.html"
}
```

## style[0]

```css

.parent {
  mask-border-source: url(../support/green.png);
}
.target {
  width: 50px;
  height: 50px;
  background-color: black;
  display: inline-block;
  border: 5px solid aqua;
  mask-border-source: url(../support/blue_color.png);
  mask-border-slice: 10%;
}
.expected {
  background-color: green;
  margin-right: 2px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “mask-border-source”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-border-source”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-border-slice”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
