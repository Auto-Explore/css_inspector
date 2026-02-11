# css/css-overflow/scroll-markers/column-scroll-marker-008.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/column-scroll-marker-008.html"
}
```

## style[0]

```css

  * {
    font-family: monospace;
  }
  .scroller {
    columns: 1;
    overflow: hidden;
    column-fill: auto;
    height: 100px;
    scroll-marker-group: before;
  }
  .scroller::scroll-marker-group {
    float: left;
    width: 2em;
    height: 1.7em;
    overflow: hidden;
  }
  .scroller::column::scroll-marker {
    content: "S";
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
