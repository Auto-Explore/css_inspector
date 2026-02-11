# css/css-variables/variable-reference-18.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-18.html"
}
```

## style[0]

```css

body {
  color: green;
}
p {
  color: red;
  color: { [ var(--a) ] };
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
