# css/css-variables/variable-supports-18.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-supports-18.html"
}
```

## style[0]

```css

body { color: red; }
@supports (color: var(--a) <!--) {
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
