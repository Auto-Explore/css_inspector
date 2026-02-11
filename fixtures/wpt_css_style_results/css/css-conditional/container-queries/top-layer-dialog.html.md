# css/css-conditional/container-queries/top-layer-dialog.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/top-layer-dialog.html"
}
```

## style[0]

```css

  #container {
    container-type: inline-size;
  }
  dialog {
    color: red;
  }
  @container (max-width: 200px) {
    dialog { color: green; }
  }
  @container (max-width: 100px) {
    dialog { color: lime; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
