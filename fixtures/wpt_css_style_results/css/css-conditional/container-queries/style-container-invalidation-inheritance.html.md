# css/css-conditional/container-queries/style-container-invalidation-inheritance.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/style-container-invalidation-inheritance.html"
}
```

## style[0]

```css

  #container {
    container-name: container;
  }
  .match {
    --match: true;
  }
  #inner {
    color: red;
  }
  @container container style(--match: true) {
    #inner {
      color: green;
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
