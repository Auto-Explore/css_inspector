# css/css-overflow/scroll-markers/scroll-marker-group-001.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-001.html"
}
```

## style[0]

```css

  div {
    overflow: auto;
    scroll-marker-group: before;
  }

  div::scroll-marker-group {
    background: green;
    display: flex;
    height: 100px;
    width: 100px;
  }
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
