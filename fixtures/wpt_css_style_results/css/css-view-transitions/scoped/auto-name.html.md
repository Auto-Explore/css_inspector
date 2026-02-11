# css/css-view-transitions/scoped/auto-name.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/auto-name.html"
}
```

## style[0]

```css

  #container {
    display: flex;
    flex-direction: row;
    view-transition-name: auto;
    position: relative;
  }

  .item {
    background-color: teal;
    color: white;
    text-align: center;
    line-height: 50px;
    width: 50px;
    height: 50px;
    margin: 5px;
    display: inline-block;
  }

  ::view-transition-group(*) {
    animation-duration: 2s;
  }

  ::view-transition-old(*) {
    animation-name: -ua-view-transition-fade-out;
  }

  ::view-transition-new(*) {
    animation-name: -ua-view-transition-fade-in;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
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
