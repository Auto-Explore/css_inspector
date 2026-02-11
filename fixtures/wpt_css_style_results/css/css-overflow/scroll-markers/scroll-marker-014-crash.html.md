# css/css-overflow/scroll-markers/scroll-marker-014-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-014-crash.html"
}
```

## style[0]

```css

  .carousel {
    overflow: scroll;
    scroll-marker-group: after;
  }
  .carousel::scroll-marker-group {
    overflow: scroll;
    scroll-snap-type: x mandatory;
  }
  .carousel > *::scroll-marker {
    content: "x";
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
      "message": "Invalid value for property “scroll-snap-type”.",
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
