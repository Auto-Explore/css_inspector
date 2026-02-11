# css/css-variables/variable-supports-54.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-supports-54.html"
}
```

## style[0]

```css

body { color: red; }
@supports (--a: [;] var(--b)) {
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
