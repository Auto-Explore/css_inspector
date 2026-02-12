# css/css-overflow/scroll-markers/scroll-marker-group-012.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-012.html"
}
```

## style[0]

```css

  #scroller {
    overflow: hidden;
    height: 100px;
    padding: 0;
    scroll-marker-group: before;
  }

  #scroller::scroll-marker-group {
    display: flex;
    height: 200px;
    background: gray;
  }

  #scroller>div::scroll-marker {
    display: block;
    width: 25px;
    height: 50%;
    content: "";
    background: yellow;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
