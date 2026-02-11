# css/css-ui/cursor-hover-001-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/cursor-hover-001-manual.html"
}
```

## style[0]

```css

div {
  cursor: url("support/cursors/fail.png"), help;
  width: 100px;
  height: 100px;
  border: solid blue;
}
div:hover {
  cursor: crosshair;
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
