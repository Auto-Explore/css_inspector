# css/css-overflow/scroll-markers/scroll-markers-resize-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-markers-resize-crash.html"
}
```

## style[0]

```css

  .carousel {
    overflow: auto;
    width: 501px;
    scroll-marker-group: after;

    &::scroll-marker-group {
      display: block;
    }
  }

  .item {
    width: 1000px;
    height: 285px;

    &::scroll-marker {
      content: "x";
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
