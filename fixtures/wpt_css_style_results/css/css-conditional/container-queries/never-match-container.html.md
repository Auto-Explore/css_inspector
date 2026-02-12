# css/css-conditional/container-queries/never-match-container.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/never-match-container.html"
}
```

## style[0]

```css

  #outer-container {
    width: 100px;
    container-type: inline-size;
  }
  #container-inline, #svg-container {
    width: 100px;
    container-type: inline-size;
  }
  @container (width >= 0px) {
    #inner { color: red; }
    #svg-inner { fill: red; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
