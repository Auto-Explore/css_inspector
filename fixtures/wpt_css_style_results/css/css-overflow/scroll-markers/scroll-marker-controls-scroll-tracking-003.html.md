# css/css-overflow/scroll-markers/scroll-marker-controls-scroll-tracking-003.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-controls-scroll-tracking-003.html"
}
```

## style[0]

```css

  #scroller {
    direction: rtl;
    overflow: scroll;
    width: 100px;
    height: 150px;
    white-space: nowrap;
    scroll-marker-group: before;
  }

  #scroller::scroll-marker-group {
    border: 3px solid black;
    display: flex;
    width: 100px;
    height: 20px;
  }

  #scroller div {
    border: 3px solid black;
    background-color: blue;
    display: inline-block;
    margin: 10px;
    width: 100px;
    height: 100px;
  }

  #scroller div::scroll-marker {
    content: '';
    background-color: red;
    width: 10px;
    height: 10px;
    border-radius: 50%;
  }

  #scroller div::scroll-marker:target-current {
    background-color: green;
  }

  #target {
    background-color: purple;
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
