# css/css-conditional/container-queries/ineligible-containment.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/ineligible-containment.html"
}
```

## style[0]

```css

  #outer, #inner1, #inner2 {
    width: 200px;
    container-type: inline-size;
  }
  #inner1 {
    display: table;
  }
  p {
    color: green;
  }
  @container (min-width: 1px) {
    p { color: red; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
