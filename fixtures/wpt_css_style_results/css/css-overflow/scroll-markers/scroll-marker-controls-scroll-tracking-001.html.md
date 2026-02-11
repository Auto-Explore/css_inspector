# css/css-overflow/scroll-markers/scroll-marker-controls-scroll-tracking-001.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-controls-scroll-tracking-001.html"
}
```

## style[0]

```css

  #scroller {
    overflow: scroll;
    scroll-marker-group: before;
    height: 100px;
  }

  #scroller::scroll-marker-group {
    border: 3px solid black;
    display: flex;
    width: 100px;
    height: 20px;
  }

  #scroller div {
    width: 100px;
    height: 100px;
  }

  #scroller div::scroll-marker {
    content: '';
    background-color: red;
    display: inline-flex;
    width: 10px;
    height: 10px;
    border-radius: 50%;
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
