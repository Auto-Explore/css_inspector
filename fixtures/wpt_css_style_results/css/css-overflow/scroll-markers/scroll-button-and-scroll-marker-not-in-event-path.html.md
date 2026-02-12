# css/css-overflow/scroll-markers/scroll-button-and-scroll-marker-not-in-event-path.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-button-and-scroll-marker-not-in-event-path.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  #scroller {
    height: 100px;
    width: 100px;
    overflow: scroll;
    scroll-marker-group: before;
  }

  #scroller::scroll-marker-group {
    height: 10px;
  }

  #target {
    height: 100px;
    width: 200px;
  }

  #scroller::scroll-button(right) {
    content: "";
    height: 10px;
    width: 10px;
  }

  #target::scroll-marker {
    content: "";
    height: 10px;
    width: 10px;
    display: block;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
