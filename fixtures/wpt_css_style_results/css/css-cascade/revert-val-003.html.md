# css/css-cascade/revert-val-003.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-val-003.html"
}
```

## style[0]

```css

html, body { margin: 0 }
h1 {
  margin: 0;
  transition: margin 10s;
  transition-delay: -5s; /* So we can expect it to be half-way the transition when toggling the property */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
