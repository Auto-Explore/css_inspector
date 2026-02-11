# css/css-conditional/container-queries/top-layer-dialog-backdrop.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/top-layer-dialog-backdrop.html"
}
```

## style[0]

```css

  html { background: green; }
  #container { container-type: inline-size; }
  @container (max-width: 200px) {
    ::backdrop { display: none; }
    #dialog { visibility: hidden; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
