# css/css-overflow/scroll-markers/scroll-marker-group-size-container-query.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-size-container-query.html"
}
```

## style[0]

```css

  #container {
    container-type: inline-size;
    width: 400px;
  }
  #scroller {
    container-type: inline-size;
    overflow: auto;
    width: 200px;
    height: 200px;
    scroll-marker-group: before;
    @container (width = 400px) {
      &::scroll-marker-group { --test: pass; }
    }
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
