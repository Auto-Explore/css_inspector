# css/css-overflow/scroll-markers/column-scroll-marker-013.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/column-scroll-marker-013.html"
}
```

## style[0]

```css

  #container {
    overflow: hidden;
    columns: 1;
    column-fill: auto;
    gap: 0;
    width: 100px;
    height: 100px;
    scroll-marker-group: before;
  }
  #container::scroll-marker-group {
    position: relative;
    left: -200px;
    display: flex;
    height: 100px;
  }
  #container::column::scroll-marker {
    width: 100px;
    height: 100px;
    flex: none;
    content: "";
  }
  #container::column::scroll-marker:target-current {
    background: green;
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
