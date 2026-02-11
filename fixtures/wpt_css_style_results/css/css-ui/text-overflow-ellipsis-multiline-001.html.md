# css/css-ui/text-overflow-ellipsis-multiline-001.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/text-overflow-ellipsis-multiline-001.html"
}
```

## style[0]

```css

div {
  width: 6ch;
  overflow: hidden;
  text-overflow: ellipsis;
}
div::first-line {
  color: orange;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
