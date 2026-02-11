# css/css-conditional/container-queries/pseudo-elements-011.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/pseudo-elements-011.html"
}
```

## style[0]

```css

  #container {
    width: 500px;
    container-type: inline-size;
  }
  #container::highlight(hi) {
    color: red;
    background: transparent;
  }
  @container (width >= 400px) {
    #container::highlight(hi) {
      color: green;
      background: transparent;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
