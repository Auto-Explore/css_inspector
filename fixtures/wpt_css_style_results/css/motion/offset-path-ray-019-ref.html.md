# css/motion/offset-path-ray-019-ref.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-019-ref.html"
}
```

## style[0]

```css

      #container1 {
        width: 100px;
        height: 100px;
      }
      #container2 {
        width: 200px;
        height: 200px;
      }
      #target1 {
        position: relative;
        width: 50px;
        height: 50px;
        background-color: lime;
        transform-origin: 0px 0px;
        transform: translateX(100px);
      }
      #target2 {
        position: relative;
        width: 50px;
        height: 50px;
        background-color: lime;
        transform-origin: 0px 0px;
        transform: translateX(200px);
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
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
