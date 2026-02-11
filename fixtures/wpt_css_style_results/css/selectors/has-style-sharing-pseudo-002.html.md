# css/selectors/has-style-sharing-pseudo-002.html

```json
{
  "format_version": 3,
  "file": "css/selectors/has-style-sharing-pseudo-002.html"
}
```

## style[0]

```css

div {
  width: 20px;
  height: 20px;
}

:has(> span)::after {
  font: 15px/1 Ahem;
  content: 'x';
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
