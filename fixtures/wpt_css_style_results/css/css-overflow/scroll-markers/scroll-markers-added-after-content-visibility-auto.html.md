# css/css-overflow/scroll-markers/scroll-markers-added-after-content-visibility-auto.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-markers-added-after-content-visibility-auto.html"
}
```

## style[0]

```css

  .scrollmarkergroup {
    scroll-marker-group: after;
  }
  #scroller {
    width: 600px;
    height: 300px;
    overflow: scroll;
  }

  #scroller div {
    width: 600px;
    height: 300px;
    margin-bottom: 20px;
    background: green;
    content-visibility: auto;
  }

  #scroller::scroll-marker-group {
    border: 3px solid black;
    padding: 5px;
    height: 20px;
    display: block;
  }

  #scroller div::scroll-marker {
    content: "";
    width: 10px;
    height: 10px;
    background-color: blue;
    border-radius: 50%;
    display: inline-block;
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
