# css/css-overflow/scroll-markers/scroll-marker-inert-002.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-inert-002.html"
}
```

## style[0]

```css

  body { margin: 0; }
  #scroller {
    width: 100px;
    height: 100px;
    overflow-y: scroll;
    scroll-marker-group: before;
  }
  #scroller::scroll-marker-group {
    height: 100px;
  }
  .item {
    height: 100px;
    background-color: lime;
    interactivity: inert;
  }
  .item::scroll-marker {
    content: "X";
    display: block;
    width: 20px;
    height: 20px;
  }
  #filler {
    height: 400px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
