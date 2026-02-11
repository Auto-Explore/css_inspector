# css/css-variables/variable-reference-07.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-07.html"
}
```

## style[0]

```css

body {
  --a: crimson;
  color: red;
}
p {
  color: green;
  color: var(--a,;);
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
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
