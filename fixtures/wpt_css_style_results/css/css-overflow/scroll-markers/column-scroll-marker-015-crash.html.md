# css/css-overflow/scroll-markers/column-scroll-marker-015-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/column-scroll-marker-015-crash.html"
}
```

## style[0]

```css

  .carousel {
    columns: 1;
    block-size: 100px;
    overflow-x: scroll;
    scroll-marker-group: after;

    &::scroll-marker-group {
      overflow-x: scroll;
      scroll-snap-type: x mandatory;
    }
    &::column::scroll-marker {
      display: block;
      content: "x";
      scroll-snap-align: center;
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
