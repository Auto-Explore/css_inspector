# css/css-variables/variable-reference-22.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-22.html"
}
```

## style[0]

```css

p {
  color: red;
  --\d800: green;
  color: var(--\d800);
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid property name in declaration.",
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
