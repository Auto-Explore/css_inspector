# css/motion/offset-path-ray-022-ref.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-022-ref.html"
}
```

## style[0]

```css

      #container {
        width: 400px;
        height: 300px;
        overflow: scroll;
      }
      #target {
        position: relative;
        left: 200px;
        top: 100px;
        width: 100px;
        height: 50px;
        background-color: lime;
        transform-origin: 0px 0px;
        transform: rotate(90deg) translate(200px);
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
