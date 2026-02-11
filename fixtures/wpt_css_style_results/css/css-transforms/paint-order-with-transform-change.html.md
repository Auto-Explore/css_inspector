# css/css-transforms/paint-order-with-transform-change.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/paint-order-with-transform-change.html"
}
```

## style[0]

```css

  #bottom {
    will-change: transform;
    position: absolute;
    top: 0;
    left: 100px;
    width: 100px;
    height: 100px;
    background: red;
  }
  #top {
    position: absolute;
    top: 0;
    left: 0;
    width: 100px;
    height: 100px;
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
