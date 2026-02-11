# css/css-ui/cursor-006-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/cursor-006-manual.html"
}
```

## style[0]

```css

div {
  cursor: url("support/cursors/fail.png"), help;
  cursor: copy;
  width: 100px;
  height: 100px;
  border: solid blue;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “cursor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “cursor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
