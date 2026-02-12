# css/css-conditional/container-queries/nested-size-style-container-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/nested-size-style-container-invalidation.html"
}
```

## style[0]

```css

  #container {
    container-type: size;
    width: 1000px;
  }
  #container.narrow {
    width: 600px;
  }
  #target {
    color: red;
  }
  @container (width < 800px) {
    @container style(--foo: bar) {
      #target { color: green; }
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
