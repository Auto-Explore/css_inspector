# css/css-overflow/scroll-markers/column-scroll-marker-010.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/column-scroll-marker-010.html"
}
```

## style[0]

```css

  #scrollable {
    overflow: hidden;
    height: 100px;
    border: 15px solid;
    line-height: 20px;
    scroll-marker-group: before;
    background: yellow;
  }
  #multicol {
    columns: 3;
    column-fill: auto;
    gap: 0;
    orphans: 1;
    widows: 1;
    width: 420px;
    height: 100px;
    margin-left: 100px;
    margin-top: 50px;
  }
  #scrollable::scroll-marker-group {
    display: flex;
    justify-content: space-between;
    height: 50px;
    background: cyan;
  }
  #multicol::column::scroll-marker {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 50px;
    height: 50px;
    background: hotpink;
    content: "*";
  }
```

```json
{
  "errors": 5,
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
    }
  ],
  "warnings": 0
}
```
