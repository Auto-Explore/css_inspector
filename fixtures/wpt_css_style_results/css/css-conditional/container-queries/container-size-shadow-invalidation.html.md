# css/css-conditional/container-queries/container-size-shadow-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-size-shadow-invalidation.html"
}
```

## style[0]

```css

  .container {
    container-type: inline-size;
    width: 100px;
  }
  @container (width = 200px) {
    .target { color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
