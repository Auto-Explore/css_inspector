# css/css-variables/variable-reference-16.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-16.html"
}
```

## style[0]

```css

body {
  color: red;
}
p {
  color: crimson;
  color: var(--a, var(--b, var(--c, green)));
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
