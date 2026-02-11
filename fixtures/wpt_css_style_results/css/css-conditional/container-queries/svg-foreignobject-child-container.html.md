# css/css-conditional/container-queries/svg-foreignobject-child-container.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/svg-foreignobject-child-container.html"
}
```

## style[0]

```css

  svg {
    display: block;
    width: 200px;
    height: 200px;
    container-type: size;
  }
  #container {
    width: 100px;
    height: 100px;
    container-type: size;
  }
  @container (width = 100px) {
    #inner { color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
