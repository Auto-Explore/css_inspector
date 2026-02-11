# css/css-conditional/container-queries/font-relative-calc-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/font-relative-calc-dynamic.html"
}
```

## style[0]

```css

  body { font-size: 10px; }
  body.larger { font-size: 20px; }
  #container {
    container-type: inline-size;
    width: 100px;
    color: red;
  }
  #intermediate {
    font-size: 8px;
  }
  @container (width: calc(1em + 80px)) {
    #target { color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
