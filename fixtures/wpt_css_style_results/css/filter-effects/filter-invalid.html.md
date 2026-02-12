# css/filter-effects/filter-invalid.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-invalid.html"
}
```

## style[0]

```css

div, rect {
  width: 100px;
  height: 100px;
  filter: url(#not-found);
}
div {
  background-color: blue;
}
rect {
  fill: purple;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
