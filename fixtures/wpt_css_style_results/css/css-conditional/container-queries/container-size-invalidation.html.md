# css/css-conditional/container-queries/container-size-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-size-invalidation.html"
}
```

## style[0]

```css

  #container {
    container-type: size;
    width: 200px;
    height: 50px;
  }
  div { color: red; }
  @container (min-width: 300px) {
    div { color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
