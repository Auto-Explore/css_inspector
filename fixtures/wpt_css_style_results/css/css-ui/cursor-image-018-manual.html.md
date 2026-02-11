# css/css-ui/cursor-image-018-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/cursor-image-018-manual.html"
}
```

## style[0]

```css

div.test{
  background: #D2B48C; border: 2px solid #555;
  cursor: url("support/cursors/not-an-image.foo"), crosshair;
  width: 128px; height: 128px;
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
