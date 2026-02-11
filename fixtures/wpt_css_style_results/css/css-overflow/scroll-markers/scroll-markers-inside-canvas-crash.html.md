# css/css-overflow/scroll-markers/scroll-markers-inside-canvas-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-markers-inside-canvas-crash.html"
}
```

## style[0]

```css

  #scroller { overflow: scroll; scroll-marker-group: after; }
  #item::scroll-marker { content: "X" }
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
