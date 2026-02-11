# css/css-masking/animations/mask-border-slice-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/animations/mask-border-slice-interpolation.html"
}
```

## style[0]

```css

.parent {
  mask-border-slice: 50%;
}
.target {
  width: 50px;
  height: 50px;
  background-color: black;
  display: inline-block;
  border: 25px;
  mask-border-source: linear-gradient(45deg, red, blue, green);
  mask-border-slice: 20%;
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
      "message": "Unknown property “mask-border-slice”.",
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
