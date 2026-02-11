# css/css-variables/variable-supports-50.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-supports-50.html"
}
```

## style[0]

```css

body { color: red; }
@supports (--a: var(--b) <!--) {
  p { color: green; }
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
