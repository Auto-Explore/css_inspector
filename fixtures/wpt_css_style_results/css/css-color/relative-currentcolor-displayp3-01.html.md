# css/css-color/relative-currentcolor-displayp3-01.html

```json
{
  "format_version": 3,
  "file": "css/css-color/relative-currentcolor-displayp3-01.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background-color: red;
  color: red;
  background-color: color(from currentColor display-p3 r g b);
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
