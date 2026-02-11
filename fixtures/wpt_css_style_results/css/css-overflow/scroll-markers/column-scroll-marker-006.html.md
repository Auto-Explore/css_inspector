# css/css-overflow/scroll-markers/column-scroll-marker-006.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/column-scroll-marker-006.html"
}
```

## style[0]

```css

  #container {
    overflow: hidden;
    columns: 3;
    column-fill: auto;
    gap: 0;
    orphans: 1;
    widows: 1;
    height: 100px;
    border: 15px solid;
    line-height: 20px;
    scroll-marker-group: before;
    background: yellow;
  }
  #container::scroll-marker-group {
    display: flex;
    justify-content: space-between;
    height: 50px;
    background: cyan;
  }
  #container::column::scroll-marker {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 50px;
    height: 50px;
    background: hotpink;
    content: "*";
  }
  #container::column::scroll-marker:target-current {
    background: green;
  }
```

```json
{
  "errors": 6,
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
