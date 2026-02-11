# css/css-transitions/starting-style-adjustment.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/starting-style-adjustment.html"
}
```

## style[0]

```css

legend {
  transition: display 1s step-end allow-discrete;
}
@starting-style {
  legend { display:inline; }
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
