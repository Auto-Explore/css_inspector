# css/css-overflow/scroll-markers/scroll-marker-in-display-none-column-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-in-display-none-column-crash.html"
}
```

## style[0]

```css

    body {
      margin: 0;
    }
    #mc {
      overflow: hidden;
      scroll-marker-group: before;
      columns: 1;
      column-fill: auto;
      width: 400px;
      height: 100px;
    }
    #mc::scroll-marker-group {
      display: flex;
      height: 20px;
    }
    #mc::column::scroll-marker {
      width: 20px;
      content: "X";
    }
    #mc:hover::column {
      display: none;
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
