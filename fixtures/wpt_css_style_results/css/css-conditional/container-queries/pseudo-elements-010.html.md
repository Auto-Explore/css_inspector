# css/css-conditional/container-queries/pseudo-elements-010.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/pseudo-elements-010.html"
}
```

## style[0]

```css

  .container { container-type: inline-size; }
  #outer { width: 300px; }
  #inner { width: 500px; }

  #inner::highlight(hi) {
    color: red;
    background: transparent;
  }
  @container (width >= 400px) {
    #inner.green::highlight(hi) {
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
