# css/css-overflow/scroll-markers/scroll-buttons-container-query-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-buttons-container-query-crash.html"
}
```

## style[0]

```css

  #scroller {
    overflow: hidden;
    container-type: size;
  }
  #scroller::scroll-button(right) {
    content: ">";
  }
  @container (width) {
    #scroller::scroll-button(right) {
      display: block;
    }
  }
```

```json
{
  "errors": 2,
  "messages": [
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
