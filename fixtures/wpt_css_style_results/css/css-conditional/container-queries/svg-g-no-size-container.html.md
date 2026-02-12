# css/css-conditional/container-queries/svg-g-no-size-container.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/svg-g-no-size-container.html"
}
```

## style[0]

```css

  g {
    display: block;
    width: 100px;
    height: 100px;
    container-type: size;
  }

  @supports not (container-type: size) {
    text { fill: red; }
  }
  @container (width = 100px) {
    text { fill: red; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
