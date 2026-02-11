# css/css-ui/cursor-box-004.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/cursor-box-004.html"
}
```

## style[0]

```css

#container {
  position: absolute;
  background: blue;
  cursor: crosshair;
}
#test {
  margin: 100px 50px;
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
