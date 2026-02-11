# css/css-overflow/scroll-markers/scroll-marker-group-017.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-017.html"
}
```

## style[0]

```css

  #sc {
    position: absolute;
    overflow: hidden;
    scroll-marker-group: before;
  }
  #sc::scroll-marker-group {
    position: absolute;
    top: 50px;
    left: 0px;
    width: 110px;
    background: red;
  }
  #sc > div::scroll-marker {
    position: fixed;
    top: 0;
    content: "";
    width: 50px;
    height: 50px;
    background: green;
  }
  #sc > div:last-child::scroll-marker {
    right: 10px;
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
