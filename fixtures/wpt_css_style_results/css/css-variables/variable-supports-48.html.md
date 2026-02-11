# css/css-variables/variable-supports-48.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-supports-48.html"
}
```

## style[0]

```css

body { color: red; }
@supports (--a: var(--b, var(--c, var(--d, black)))) {
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
