# css/filter-effects/backdrop-filter-invalid.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-invalid.html"
}
```

## style[0]

```css

div, rect {
  width: 100px;
  height: 100px;
  backdrop-filter: url(#not-found);
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
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
