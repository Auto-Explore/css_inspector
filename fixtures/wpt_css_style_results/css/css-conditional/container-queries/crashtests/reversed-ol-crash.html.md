# css/css-conditional/container-queries/crashtests/reversed-ol-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/crashtests/reversed-ol-crash.html"
}
```

## style[0]

```css

.container {
  container-type: size;
}

/* Prevent double layout due to scrollbar speculation */
html {
  overflow: hidden;
}

@container (width > 1px) {
  .item {
    display: list-item;
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
