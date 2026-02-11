# css/css-conditional/container-queries/pseudo-elements-009.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/pseudo-elements-009.html"
}
```

## style[0]

```css

  #container {
    container-type: inline-size;
    width: 500px;
  }

  ::selection { color: red; background: transparent; }
  ::highlight(hi) { color: red; background: transparent; }

  @container (width >= 400px) {
    ::selection { color: green }
    ::highlight(hi) { color: green }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
