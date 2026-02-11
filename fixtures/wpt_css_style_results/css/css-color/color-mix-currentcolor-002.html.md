# css/css-color/color-mix-currentcolor-002.html

```json
{
  "format_version": 3,
  "file": "css/css-color/color-mix-currentcolor-002.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background-color: red;
  color: red;
  background-color: color-mix(in lch, currentColor 50%, blue);
}
div div {
  color: green;
  background-color: inherit;
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
