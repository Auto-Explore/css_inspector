# css/css-overflow/scroll-markers/scroll-marker-group-size-container-query-root.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-size-container-query-root.html"
}
```

## style[0]

```css

  :root {
    container-type: inline-size;
    width: 400px;
    scroll-marker-group: before;
    @container (width = 400px) {
      &::scroll-marker-group { --test: pass; }
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
