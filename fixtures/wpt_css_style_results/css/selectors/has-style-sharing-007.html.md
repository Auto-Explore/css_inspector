# css/selectors/has-style-sharing-007.html

```json
{
  "format_version": 3,
  "file": "css/selectors/has-style-sharing-007.html"
}
```

## style[0]

```css

.cousin {
  width: 1em;
  height: 1em;
  background: purple;
}
.special.cousin:not(:has(span)) {
  background: blue;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
