# css/css-overflow/scroll-markers/scroll-marker-005.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-005.html"
}
```

## style[0]

```css

  .container {
    scroll-marker-group: before;
    overflow: hidden;
  }

  .container::scroll-marker-group {
    display: flex;
    height: 100px;
  }

  .marker::scroll-marker {
    display: block;
    width: 20px;
    height: 100px;
    content: "";
    background: green;
  }

  .contain_abs {
    position: relative;
  }

  .contain_all {
    contain: layout;
  }

  .uncontained::scroll-marker {
    z-index: 100;
    content: "ERROR";
    background: red;
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
