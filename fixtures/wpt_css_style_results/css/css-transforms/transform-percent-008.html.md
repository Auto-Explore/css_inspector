# css/css-transforms/transform-percent-008.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-percent-008.html"
}
```

## style[0]

```css

      div {
        width: 60px;
        height: 10px;
        padding: 10px;
        border: 10px solid gold;
        margin: 10px;
        background: gold;
        position: absolute;
        left: 90px;
        top: 90px;
        transform: rotate(10deg) translatex(50%) rotate(10deg) translatey(100%)
                   skewx(10deg) translate(25px, 25px);
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
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
