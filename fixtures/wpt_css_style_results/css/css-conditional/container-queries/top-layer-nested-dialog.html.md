# css/css-conditional/container-queries/top-layer-nested-dialog.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/top-layer-nested-dialog.html"
}
```

## style[0]

```css

  dialog { color: red; }
  #container { width: 100px; }
  #container, #outer { container-type: inline-size; }
  @container (min-width: 200px) {
    #outer { width: 400px; color: lime; }
  }
  @container (min-width: 400px) {
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
