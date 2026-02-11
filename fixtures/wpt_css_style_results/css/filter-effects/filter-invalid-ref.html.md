# css/filter-effects/filter-invalid-ref.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-invalid-ref.html"
}
```

## style[0]

```css

div, rect {
  width: 100px;
  height: 100px;
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
