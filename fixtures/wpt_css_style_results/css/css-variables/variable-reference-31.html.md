# css/css-variables/variable-reference-31.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-31.html"
}
```

## style[0]

```css

body {
  color: orange;
}
p {
  color: red;
  --0: green;
  color: var(--0);
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
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
