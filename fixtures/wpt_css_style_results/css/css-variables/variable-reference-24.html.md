# css/css-variables/variable-reference-24.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-24.html"
}
```

## style[0]

```css

p {
  color: red;
  --\ffffff: green;
  color: var(--\fffd);
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
