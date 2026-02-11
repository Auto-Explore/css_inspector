# css/css-overflow/chrome-480554290-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/chrome-480554290-crash.html"
}
```

## style[0]

```css

  #scroller {
    overflow: auto;
    scroll-marker-group: after;

    .item {
      &::scroll-marker {
        content: "";
      }
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
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
