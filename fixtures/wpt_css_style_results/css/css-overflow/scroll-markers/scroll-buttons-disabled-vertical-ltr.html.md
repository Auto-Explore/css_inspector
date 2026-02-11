# css/css-overflow/scroll-markers/scroll-buttons-disabled-vertical-ltr.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-buttons-disabled-vertical-ltr.html"
}
```

## style[0]

```css

  * {
    margin: 0;
    padding: 0;
  }

  #scroller {
    width: 600px;
    height: 300px;
    overflow: auto;
    scroll-marker-group: after;
    white-space: nowrap;
    writing-mode: vertical-lr;
  }

  #scroller div {
    background: green;
    display: inline-block;
    width: 600px;
    height: 270px;
  }

  #scroller::scroll-marker-group {
    border: 3px solid black;
    padding: 5px;
    display: flex;
    height: 20px;
    width: 40px;
  }

  #scroller::scroll-button(inline-end) {
    content: ">";
    background: blue;
    display: flex;
    height: 20px;
    width: 20px;
  }

  #scroller::scroll-button(inline-end):disabled {
    background: gray;
  }

  #scroller div::scroll-marker {
    content: "";
    width: 10px;
    height: 10px;
    background-color: blue;
    border-radius: 100%;
    display: inline-block;
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
