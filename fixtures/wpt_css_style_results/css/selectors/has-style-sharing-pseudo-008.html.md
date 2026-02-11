# css/selectors/has-style-sharing-pseudo-008.html

```json
{
  "format_version": 3,
  "file": "css/selectors/has-style-sharing-pseudo-008.html"
}
```

## style[0]

```css

::marker {
  font: 15px/1 Ahem;
}

:has(> span)::marker {
  content: '';
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
