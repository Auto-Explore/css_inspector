# css/css-conditional/container-queries/orthogonal-wm-container-query.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/orthogonal-wm-container-query.html"
}
```

## style[0]

```css

  #container {
    container-type: size;
    width: 50vw;
    height: 50vh;
  }
  #orthogonal {
    font: 50px/1 Ahem;
  }
  @container (max-width: 100px) {
    #orthogonal {
      writing-mode: vertical-lr;
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
