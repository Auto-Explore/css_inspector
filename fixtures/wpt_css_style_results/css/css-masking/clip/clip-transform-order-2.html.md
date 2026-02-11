# css/css-masking/clip/clip-transform-order-2.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip/clip-transform-order-2.html"
}
```

## style[0]

```css

  body { margin: 0; overflow: hidden; }
  #testcase {
    position: fixed;
    left: -100px;
    width: 400px;
    height: 400px;
    background: green;
    transform: translateX(110px);
    clip: rect(0px, 200px, 200px, 0px);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
