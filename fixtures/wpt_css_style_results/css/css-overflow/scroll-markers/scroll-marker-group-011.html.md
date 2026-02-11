# css/css-overflow/scroll-markers/scroll-marker-group-011.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-011.html"
}
```

## style[0]

```css

  #scroller {
    overflow: hidden;
    height: 100px;
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
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
