# css/css-variables/variable-reference-17.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-17.html"
}
```

## style[0]

```css

body {
  color: red;
}
p {
  color: crimson;
  --a: green;
  color: var(--a, <!--);
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
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
