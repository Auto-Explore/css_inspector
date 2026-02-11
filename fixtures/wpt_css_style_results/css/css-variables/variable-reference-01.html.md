# css/css-variables/variable-reference-01.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-01.html"
}
```

## style[0]

```css

:root {
  --a: green;
}
p {
  color: var(--a);
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
