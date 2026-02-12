# css/css-overflow/scroll-markers/scroll-marker-group-016.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-016.html"
}
```

## style[0]

```css

  #sc {
    anchor-name: --a;
    overflow: hidden;
    width: 200px;
    height: 50px;
    scroll-marker-group: before;
  }
  #sc::scroll-marker-group {
    position: absolute;
    position-anchor: --a;
    position-area: bottom center;
    background: red;
  }
  #sc > div::scroll-marker {
    float: left;
    content: "";
    width: 50px;
    height: 50px;
    background: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
