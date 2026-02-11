# css/css-color/relative-currentcolor-xyzd65-01.html

```json
{
  "format_version": 3,
  "file": "css/css-color/relative-currentcolor-xyzd65-01.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background-color: red;
  color: red;
  background-color: color(from currentColor xyz-d65 x y z);
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
