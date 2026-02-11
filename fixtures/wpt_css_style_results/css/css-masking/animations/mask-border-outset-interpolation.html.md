# css/css-masking/animations/mask-border-outset-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/animations/mask-border-outset-interpolation.html"
}
```

## style[0]

```css

.parent {
  mask-border-outset: 10px;
}
.target {
  width: 50px;
  height: 50px;
  background-color: black;
  display: inline-block;
  border: 25px;
  margin-right: 50px;
  mask-border-slice: 30%;
  background-clip: content-box;
  mask-border-source: linear-gradient(45deg, pink, blue, white, black, green);
  mask-border-outset: 1px;
}
.expected {
  background-color: green;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “mask-border-outset”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-border-slice”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-border-source”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-border-outset”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
