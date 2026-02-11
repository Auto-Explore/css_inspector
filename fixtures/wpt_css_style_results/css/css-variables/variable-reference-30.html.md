# css/css-variables/variable-reference-30.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-30.html"
}
```

## style[0]

```css

p {
  color: green;
  --a: red;
  color: var(--a) !important !important;
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
