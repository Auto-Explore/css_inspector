# css/css-variables/variable-reference-34.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-34.html"
}
```

## style[0]

```css

body {
  color: orange;
}
p {
  color: green;
  --a: red;
  color: var(--a, url("
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
