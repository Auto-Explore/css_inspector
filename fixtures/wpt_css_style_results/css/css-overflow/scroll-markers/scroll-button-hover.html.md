# css/css-overflow/scroll-markers/scroll-button-hover.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-button-hover.html"
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
    white-space: nowrap;
  }

  #scroller div {
    background: green;
    display: inline-block;
    width: 600px;
    height: 270px;
  }

  #scroller::scroll-button(inline-end) {
    content: ">";
    width: 100px;
    height: 100px;
    position: absolute;
    top: 0;
    left: 0;
    background-color: green;
    display: inline-block;
  }

  #scroller::scroll-button(inline-end):hover {
    background-color: blue;
  }
```

```json
{
  "errors": 2,
  "messages": [
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
