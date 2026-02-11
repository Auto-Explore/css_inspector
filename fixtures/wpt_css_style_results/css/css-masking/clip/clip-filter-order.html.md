# css/css-masking/clip/clip-filter-order.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip/clip-filter-order.html"
}
```

## style[0]

```css

  body { margin: 0 }
  #testcase {
    position: absolute;
    left: 10px;
    width: 400px;
    height: 400px;
    background: green;
    will-change: transform;
    filter: drop-shadow(16px 16px 20px red);
    clip: rect(0px, 200px, 200px, 0px);
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
