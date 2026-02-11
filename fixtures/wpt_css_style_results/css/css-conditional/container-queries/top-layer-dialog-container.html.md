# css/css-conditional/container-queries/top-layer-dialog-container.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/top-layer-dialog-container.html"
}
```

## style[0]

```css

  #parent { width: 100px; }
  #dialog {
    container-type: inline-size;
    width: auto;
    border: none;
  }
  #child { color: red; }
  @container (min-width: 200px) {
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
