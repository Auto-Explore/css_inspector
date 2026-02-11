# css/css-variables/variable-supports-13.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-supports-13.html"
}
```

## style[0]

```css

body { color: red; }
@supports (color: something 3px url(whereever) calc(var(--a) + 1px)) {
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
