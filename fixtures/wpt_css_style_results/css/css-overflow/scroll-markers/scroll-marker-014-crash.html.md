# css/css-overflow/scroll-markers/scroll-marker-014-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-014-crash.html"
}
```

## style[0]

```css

  .carousel {
    overflow: scroll;
    scroll-marker-group: after;
  }
  .carousel::scroll-marker-group {
    overflow: scroll;
    scroll-snap-type: x mandatory;
  }
  .carousel > *::scroll-marker {
    content: "x";
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
