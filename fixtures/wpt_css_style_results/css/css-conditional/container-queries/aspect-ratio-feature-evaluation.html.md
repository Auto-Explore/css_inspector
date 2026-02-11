# css/css-conditional/container-queries/aspect-ratio-feature-evaluation.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/aspect-ratio-feature-evaluation.html"
}
```

## style[0]

```css

  .container {
    width: 100px;
    height: 100px;
  }
  #inline-size { container-type: inline-size; }
  #size { container-type: size; }
  span { color: red }
  @container (min-aspect-ratio: 1 / 1000) {
    span { color: green; }
  }
  @container (min-aspect-ratio: 2 / 1) {
    span { background-color: lime; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
