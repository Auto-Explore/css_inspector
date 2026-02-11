# css/css-overflow/scroll-markers/column-scroll-marker-reattach-target-current.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/column-scroll-marker-reattach-target-current.html"
}
```

## style[0]

```css

  * {
    font-family: monospace;
  }

  #scroller {
    overflow: hidden;
    scroll-marker-group: before;
    columns: 1;
    gap: 0;
    height: 100px;
    width: 100px;
  }
  #scroller::scroll-marker-group {
    display: block;
    height: 100px;
  }
  #scroller::column::scroll-marker {
    display: flex;
    content: "PASS";
  }
  #scroller::column::scroll-marker:target-current {
    display: block;
  }
  #item {
    height: 100px;
    width: 100px;
  }
```

```json
{
  "errors": 4,
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
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
