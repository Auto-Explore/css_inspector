# css/css-color/color-mix-currentcolor-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-color/color-mix-currentcolor-002-ref.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background-color: red;
  color: red;
  background-color: color-mix(in lch, green 50%, blue);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
