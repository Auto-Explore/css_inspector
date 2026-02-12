# css/css-overflow/scroll-markers/scroll-marker-group-add-dynamic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-add-dynamic-001.html"
}
```

## style[0]

```css

  #scroller {
    scroll-marker-group: before;
    overflow: auto;
    width: 200px;
    height: 200px;
  }

  #item::scroll-marker {
    content: "PASS"
  }

  #scroller::scroll-marker-group {
    display: flex;
  }

  #scroller.group::scroll-marker-group {
    display: block;
    height: 100px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
