# css/css-overflow/scroll-markers/nested-scroll-markers-under-content-visibility-auto-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/nested-scroll-markers-under-content-visibility-auto-ref.html"
}
```

## style[0]

```css

  #scroller {
    width: 600px;
    height: 300px;
    overflow: scroll;
    scroll-marker-group: after;
  }

  .subscroller {
    scroll-marker-group: after;
    overflow: scroll;
    width: 600px;
    height: 100px;
  }

  .cv-placeholder {
    contain: strict;
    width: 100px;
    height: 100px;
  }

  #scroller::scroll-marker-group {
    border: 3px solid black;
    padding: 5px;
    height: 20px;
    display: block;
  }

  .item {
    height: 200px;
  }

  .item::scroll-marker {
    content: "";
    width: 10px;
    height: 10px;
    background-color: blue;
    border-radius: 50%;
    display: inline-block;
  }

  .spacer {
    height: 10000px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
