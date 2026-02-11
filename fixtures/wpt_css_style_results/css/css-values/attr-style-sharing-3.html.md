# css/css-values/attr-style-sharing-3.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-style-sharing-3.html"
}
```

## style[0]

```css

  span {
    background-color: attr(data-foo type(<color>));
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
