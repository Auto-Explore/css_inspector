# css/css-variables/variable-supports-16.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-supports-16.html"
}
```

## style[0]

```css

body { color: red; }
@supports (color: var(--a, var(--b, var(--c, black)))) {
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
