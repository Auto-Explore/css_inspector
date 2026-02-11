# css/css-overflow/scroll-markers/column-scroll-marker-011.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/column-scroll-marker-011.html"
}
```

## style[0]

```css

  #container {
    overflow: hidden;
    columns: 3;
    column-fill: auto;
    gap: 0;
    height: 100px;
    scroll-marker-group: before;
  }
  #container::scroll-marker-group {
    overflow: auto;
    display: flex;
    width: 100px;
    height: 100px;
    background: red;
  }
  #container::column::scroll-marker {
    width: 25px;
    height: 100px;
    flex: none;
    content: "";
    background: green;
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
