# css/css-variables/variable-reference-20.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-20.html"
}
```

## style[0]

```css

p {
  color: red;
  --a: green;
  color: VAR(--a);
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
