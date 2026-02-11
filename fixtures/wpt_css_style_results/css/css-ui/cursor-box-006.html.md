# css/css-ui/cursor-box-006.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/cursor-box-006.html"
}
```

## style[0]

```css

#container {
  position: absolute;
  padding: 50px;
  cursor: crosshair;
}
#test{
  border-image: linear-gradient(blue, blue);
  border-image-outset: 50px;
  border-image-width: 50px;
  border-style: solid;
  border-width: 5px;
  border-color: white;
  cursor: url("support/cursors/fail.png"), help;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “cursor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
