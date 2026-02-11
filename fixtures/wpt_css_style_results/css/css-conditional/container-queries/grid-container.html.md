# css/css-conditional/container-queries/grid-container.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/grid-container.html"
}
```

## style[0]

```css

  #grid {
    display: grid;
    container-type: inline-size;
    width: 400px;
    grid-template-columns: 1fr 1fr;
  }
  @container (width = 400px) {
    #grid div { color: green }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
