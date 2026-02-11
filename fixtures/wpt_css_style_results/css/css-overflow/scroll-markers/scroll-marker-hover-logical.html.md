# css/css-overflow/scroll-markers/scroll-marker-hover-logical.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-hover-logical.html"
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
    height: 270px;
  }

  #scroller::scroll-marker-group {
    display: flex;
    height: 20px;
    width: 40px;
  }

  #scroller div::scroll-marker {
    content: "";
    width: 100px;
    height: 20px;
    display: inline-block;
  }

  #scroller div::scroll-marker:not(:hover) {
    background-color: green;
  }

  #scroller div::scroll-marker:is(:hover) {
    background-color: blue;
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
