# css/css-overflow/scroll-markers/scroll-marker-group-style-remove.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-style-remove.html"
}
```

## style[0]

```css

  #scroller {
    overflow: hidden;
    scroll-marker-group: after;
    height: 100px;
    width: 100px;
  }

  .scroll-marker-group::scroll-marker-group {
    border: 3px solid black;
    display: flex;
    height: 50px;
    width: 100px;
  }

  .item {
    height: 100px;
    width: 100px;
  }

  .item::scroll-marker {
    content: ' ';
    border-radius: 50%;
    background: blue;
    width: 50px;
    height: 50px;
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
