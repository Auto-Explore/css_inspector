# css/css-borders/tentative/border-shape/border-shape-hit-test-overflow.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-hit-test-overflow.html"
}
```

## style[0]

```css

  #wrapper {
    width: 100px;
    height: 100px;
  }
  #bs-target {
    width: 200px;
    height: 200px;
    border-shape: circle(50% at 50% 50%);
    border: 20px solid purple;
    background: green;
  }

  #bs-target:hover {
    border-color: orange;
  }

  #overflower {
    width: 400px;
    height: 25px;
    background: lightblue;
    text-align: end;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
