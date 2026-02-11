# css/css-variables/variable-reference-09.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-09.html"
}
```

## style[0]

```css

body {
  --a: green;
  color: crimson;
}
p {
  color: red;
  color: var(--a,(;));
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
