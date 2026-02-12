# css/css-overflow/scroll-markers/scroll-marker-group-011-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-011-ref.html"
}
```

## style[0]

```css

  #scroller {
    height: 100px;
  }

  #scroller::scroll-marker-group {
    display: flex;
    height: 100px;
    background: gray;
  }

  #scroller>div::scroll-marker {
    display: block;
    width: 25px;
    height: 100%;
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
