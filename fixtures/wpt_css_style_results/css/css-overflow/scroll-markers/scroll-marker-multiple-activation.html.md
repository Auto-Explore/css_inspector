# css/css-overflow/scroll-markers/scroll-marker-multiple-activation.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-multiple-activation.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  #scroller {
    width: 600px;
    height: 300px;
    overflow: auto;
    scroll-marker-group: before;
    white-space: nowrap;
  }

  #scroller div {
    background: green;
    display: inline-block;
    width: 600px;
    height: 200px;
  }

  #scroller::scroll-marker-group {
    display: flex;
    height: 20px;
    width: 140px;
  }

  #scroller div::scroll-marker {
    content: "";
    width: 20px;
    height: 20px;
    background-color: red;
    display: inline-block;
  }

  #scroller div::scroll-marker:target-current {
    background-color: green;
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
