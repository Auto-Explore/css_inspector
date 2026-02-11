# css/motion/offset-path-ray-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-003-ref.html"
}
```

## style[0]

```css

      #container {
        width: 400px;
        height: 400px;
      }
      #target {
        position: relative;
        left: 100px;
        top: 100px;
        width: 100px;
        height: 50px;
        background-color: lime;
        transform-origin: 0px 0px;
        transform: rotate(45deg) translate(100px);
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
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
