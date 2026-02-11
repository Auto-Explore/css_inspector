# css/css-variables/variable-supports-45.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-supports-45.html"
}
```

## style[0]

```css

body { color: red; }
@supports (--a: something 3px url(whereever) calc(var(--b) + 1px)) {
  p { color: green; }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
