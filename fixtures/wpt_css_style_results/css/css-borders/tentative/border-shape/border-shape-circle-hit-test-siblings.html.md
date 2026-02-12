# css/css-borders/tentative/border-shape/border-shape-circle-hit-test-siblings.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-circle-hit-test-siblings.html"
}
```

## style[0]

```css

  .container {
    position: relative;
    width: 200px;
    height: 100px;
  }
  .sibling {
    position: absolute;
    width: 100px;
    height: 100px;
    top: 0;
    left: 0;
    background: blue;
    opacity: 0.5;
  }
  #target {
    position: absolute;
    left: 50px;
    width: 80px;
    height: 80px;
    border-shape: circle(45px at 50% 50%);
    border: 10px solid purple;
    background: green;
  }
  .sibling2 {
    position: absolute;
    left: 100px;
    width: 100px;
    height: 100px;
    background: red;
    opacity: 0.5;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
