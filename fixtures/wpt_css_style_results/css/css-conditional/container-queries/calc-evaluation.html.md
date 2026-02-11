# css/css-conditional/container-queries/calc-evaluation.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/calc-evaluation.html"
}
```

## style[0]

```css

  :root { font-size: 10px; }

  /* To make output more readable */
  :root > * { font-size: initial; }

  #container {
    container-type: size;
    width: 200px;
    height: 50px;
  }
  @container (width = calc(100px + 10rem)) {
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
