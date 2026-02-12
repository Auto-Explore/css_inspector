# css/css-sizing/contain-intrinsic-size/auto-013.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/auto-013.html"
}
```

## style[0]

```css

  #t1 {
    position: absolute;
    left: 0vmax;
    content-visibility: auto;
    contain-intrinsic-size: 1000vmax 1000vmax;
    background: red;
  }
  #t1::before {
    content: "";
    display: block;
    width: 10px;
    height: 10px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
