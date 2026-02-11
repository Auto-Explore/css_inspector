# css/css-variables/variable-supports-64.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-supports-64.html"
}
```

## style[0]

```css

body { color: red; }
@supports (--a: a) and (not (--a: var(1px))) {
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
