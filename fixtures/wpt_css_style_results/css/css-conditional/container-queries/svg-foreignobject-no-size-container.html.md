# css/css-conditional/container-queries/svg-foreignobject-no-size-container.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/svg-foreignobject-no-size-container.html"
}
```

## style[0]

```css

  foreignObject {
    display: block;
    width: 100px;
    height: 100px;
    container-type: size;
  }
  @supports not (container-type: size) {
    div { color: red; }
  }
  @container (width = 100px) {
    div { color: red; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
