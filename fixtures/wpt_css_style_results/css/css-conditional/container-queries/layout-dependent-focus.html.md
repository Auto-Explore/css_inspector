# css/css-conditional/container-queries/layout-dependent-focus.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/layout-dependent-focus.html"
}
```

## style[0]

```css

  #container {
    container-type: inline-size;
    width: 200px;
  }
  #container.narrow {
    width: 100px;
  }
  @container (width = 100px) {
    #inner.hide { visibility: hidden; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
