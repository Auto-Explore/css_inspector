# css/css-overflow/scroll-markers/scroll-marker-group-019.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-019.html"
}
```

## style[0]

```css

  body {
    overflow: hidden;
  }
  #sc {
    position: fixed;
    margin-top: 50px;
    width: 100px;
    overflow: hidden;
    scroll-marker-group: after;
  }
  #sc::scroll-marker-group {
    position: absolute;
    left: 0px;
    width: 110px;
    background: red;
  }
  #sc > div::scroll-marker {
    position: absolute;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
