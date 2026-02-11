# css/css-overflow/scroll-markers/chrome-421199213-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/chrome-421199213-crash.html"
}
```

## style[0]

```css

  #scroller {
    overflow: scroll;
    scroll-marker-group: before;
  }
  #item::scroll-marker { content: counter(x); }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “scroll-marker-group”.",
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
