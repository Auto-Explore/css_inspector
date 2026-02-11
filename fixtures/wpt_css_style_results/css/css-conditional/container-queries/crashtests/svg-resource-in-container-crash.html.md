# css/css-conditional/container-queries/crashtests/svg-resource-in-container-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/crashtests/svg-resource-in-container-crash.html"
}
```

## style[0]

```css

  .cq {
    container-name: cq;
    container-type: inline-size;
  }
  @container cq (min-width: 1px) {
    .cq {
      border: 2px solid blue;
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
