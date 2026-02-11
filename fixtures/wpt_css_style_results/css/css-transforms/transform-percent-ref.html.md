# css/css-transforms/transform-percent-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-percent-ref.html"
}
```

## style[0]

```css

      div {
        width: 100px;
        height: 50px;
        background: gold;
        position: absolute;
        left: 100px;
        top: 100px;
        transform: rotate(10deg) translatex(50px) rotate(10deg)
                   translatey(50px) skewx(10deg) translate(25px, 25px);
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
