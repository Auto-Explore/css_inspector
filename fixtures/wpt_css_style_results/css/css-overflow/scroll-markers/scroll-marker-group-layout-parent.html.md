# css/css-overflow/scroll-markers/scroll-marker-group-layout-parent.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-layout-parent.html"
}
```

## style[0]

```css

  #sc {
    z-index: 1;
    overflow: hidden;
    scroll-marker-group: before;
    margin-left: -100px;
    width: 100px;
    height: 100px;
    background: red;
  }
  #sc::scroll-marker-group {
    z-index: 2;
    width: 100px;
    height: 100px;
    background: green;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “scroll-marker-group”.",
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
