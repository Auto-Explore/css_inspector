# css/css-variables/variable-supports-24.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-supports-24.html"
}
```

## style[0]

```css

body { color: red; }
@supports (color: var(--a,(;))) {
  p { color: green; }
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
