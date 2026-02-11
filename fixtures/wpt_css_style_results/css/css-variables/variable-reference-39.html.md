# css/css-variables/variable-reference-39.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-39.html"
}
```

## style[0]

```css

p {
  color: red;
  --a: var(--a, red);
  color: var(--a, green);
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
