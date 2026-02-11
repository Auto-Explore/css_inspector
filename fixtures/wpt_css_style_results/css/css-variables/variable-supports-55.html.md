# css/css-variables/variable-supports-55.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-supports-55.html"
}
```

## style[0]

```css

body { color: red; }
@supports (--a: a) and (not (--a: var(--b);)) {
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
