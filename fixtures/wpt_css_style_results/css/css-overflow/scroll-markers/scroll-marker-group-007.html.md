# css/css-overflow/scroll-markers/scroll-marker-group-007.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-007.html"
}
```

## style[0]

```css

  div::scroll-marker-group {
    display: flex;
    height: 50px;
    width: 100px;
  }

  div.scroll-marker-group-before {
    overflow: auto;
    scroll-marker-group: before;
  }

  div.scroll-marker-group-before::scroll-marker-group {
    background: green;
  }

  div.scroll-marker-group-after {
    overflow: auto;
    scroll-marker-group: after;
  }

  div.scroll-marker-group-after::scroll-marker-group {
    background: yellow;
  }
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
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
