# css/css-ui/cursor-auto-001-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/cursor-auto-001-manual.html"
}
```

## style[0]

```css

div {
  cursor: url("support/cursors/fail.png"), help;
  cursor: auto;
  color: blue;
}
p {
  cursor: text;
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
