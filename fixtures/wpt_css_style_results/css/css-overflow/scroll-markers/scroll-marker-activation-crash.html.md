# css/css-overflow/scroll-markers/scroll-marker-activation-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-activation-crash.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  #marker-scroller {
    height: 300px;
    overflow: auto;
    scroll-snap-type: x mandatory;
    position: relative;
    scroll-marker-group: before;
    white-space: nowrap;
  }

  #marker-scroller div {
    scroll-snap-align: start;
    box-sizing: border-box;
    border-radius: 5px;
    background: lightgray;
    border: 1px solid black;
    display: inline-block;
    width: 500px;
    height: 100%;
  }

  #marker-scroller::scroll-marker-group {
    display: flex;
    height: 100px;
  }

  #marker-scroller div::scroll-marker {
    content: "";
    width: 100px;
    height: 100px;
    box-sizing: border-box;
    border-radius: 5px;
    border: 1px solid black;
    display: inline-block;
  }
  #marker-scroller div::scroll-marker:target-current {
    background: blue;
  }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “scroll-marker-group”.",
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
