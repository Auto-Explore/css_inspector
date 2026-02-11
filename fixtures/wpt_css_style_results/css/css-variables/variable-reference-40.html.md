# css/css-variables/variable-reference-40.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-40.html"
}
```

## style[0]

```css

p {
  --orange: orange;
  border: 2px solid transparent;
  border-image: linear-gradient(to right, var(--orange), blue) 1 1;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
