# css/motion/offset-path-ray-contain-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-contain-004-ref.html"
}
```

## style[0]

```css

      #container {
        width: 300px;
        height: 300px;
      }
      #target {
        position: relative;
        left: 100px;
        top: 100px;
        width: 100px;
        height: 100px;
        background-color: lime;
        /* ray length is sqrt(150^2 + 0^2); contain does -max(100, 100) / 2; */
        /* the result length is 100. sin(45deg) * length = 70.71; */
        transform: rotate(-45deg) translate(100px);
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
