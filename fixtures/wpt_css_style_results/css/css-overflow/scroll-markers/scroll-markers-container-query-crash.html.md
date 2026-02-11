# css/css-overflow/scroll-markers/scroll-markers-container-query-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-markers-container-query-crash.html"
}
```

## style[0]

```css

  #scroller {
    overflow: hidden;
    scroll-marker-group: before;
    container-type: size;
  }

  #scroller>div::scroll-marker {
    content: ".";
  }

  @container (width) {
    #scroller::scroll-marker-group {
      display: block;
    }
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
