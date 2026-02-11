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
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
