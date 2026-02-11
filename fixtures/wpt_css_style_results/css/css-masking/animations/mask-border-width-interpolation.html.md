# css/css-masking/animations/mask-border-width-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/animations/mask-border-width-interpolation.html"
}
```

## style[0]

```css

.parent {
  mask-border-width: 100px;
}
.target {
  width: 80px;
  height: 80px;
  background-color: black;
  display: inline-block;
  border: 10px;
  mask-border-source: linear-gradient(45deg, red, blue, green);
  mask-border-width: 10px;
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
      "message": "Unknown property “mask-border-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-border-source”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-border-width”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
