# css/css-conditional/container-queries/container-size-invalidation-after-load.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-size-invalidation-after-load.html"
}
```

## style[0]

```css

  #container {
    container-type: size;
    width: 200px;
    height: 4em;
    border: 1px solid black;
  }
  @container (width > 300px) {
    #child { color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
