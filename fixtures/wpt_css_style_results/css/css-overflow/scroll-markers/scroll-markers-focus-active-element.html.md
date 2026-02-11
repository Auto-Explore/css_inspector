# css/css-overflow/scroll-markers/scroll-markers-focus-active-element.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-markers-focus-active-element.html"
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

  #scroller :first-child {
    background: purple;
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
    background-color: blue;
    display: inline-block;
  }

  #scroller div:first-of-type::scroll-marker {
    background-color: brown;
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
