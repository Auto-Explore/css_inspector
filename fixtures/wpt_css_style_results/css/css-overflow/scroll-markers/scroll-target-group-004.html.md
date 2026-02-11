# css/css-overflow/scroll-markers/scroll-target-group-004.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-target-group-004.html"
}
```

## style[0]

```css

  html {
    container: my-container / size;
  }

  #wrapper {
    scroll-target-group: auto;
  }

  #scroller {
    overflow: auto;
    height: 130px;
    width: 100px;
  }

  #scroller div {
    width: 100px;
    height: 100px;
    background-color: blue;
    margin: 5px;
  }

  a {
    color: red;
  }

  a:target-current {
    color: green;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “container”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “scroll-target-group”.",
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
