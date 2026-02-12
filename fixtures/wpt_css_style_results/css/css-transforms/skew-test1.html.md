# css/css-transforms/skew-test1.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/skew-test1.html"
}
```

## style[0]

```css

  .green_container {
    background-color: green;
    height: 100px;
    width: 100px;
    position: relative;
  }

  .skew_div {
    position: absolute;
    width: 40px;
    height: 40px;
    box-sizing: border-box;
    transform: skew(45deg,135deg);
    transform-origin:0 0;
  }
  .front {
    /* This one is positioned in front of a red reference polygon: */
    top: 50px;
    left: 10px;
    background: green;
    z-index: 1;
  }
  .back {
    /* This one is positioned in back of a white reference polygon: */
    top: 50px;
    left: 110px;
    background: orange;
    z-index: -1;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
