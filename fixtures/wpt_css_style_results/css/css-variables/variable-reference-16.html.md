# css/css-variables/variable-reference-16.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-16.html"
}
```

## style[0]

```css

body {
  color: red;
}
p {
  color: crimson;
  color: var(--a, var(--b, var(--c, green)));
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
