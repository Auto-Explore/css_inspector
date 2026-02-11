# css/css-overflow/scroll-markers/scroll-marker-dynamic-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-dynamic-crash.html"
}
```

## style[0]

```css

  #container {
    columns: 1;
    width: 500px;
    overflow: auto;
    scroll-marker-group: after;
  }

  #container::scroll-marker-group {
    display: block;
  }

  #container::column::scroll-marker {
    content: ' ';
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
