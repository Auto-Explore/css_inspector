# css/css-variables/variable-reference-33.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-33.html"
}
```

## style[0]

```css

body {
  color: orange;
}
p {
  color: red;
  --a: green;
  color: var(--a, "
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
