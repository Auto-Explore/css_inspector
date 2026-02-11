# css/css-shapes/shape-outside/shape-image/gradients/shape-outside-linear-gradient-004.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-image/gradients/shape-outside-linear-gradient-004.html"
}
```

## style[0]

```css

        .container {
            position: absolute;
            top: 70px;
            width: 200px;
            font-size: 0px;
        }
        .square {
            display: inline-block;
            width: 100px;
            height: 100px;
            line-height: 100px;
            background-color: green;
        }
        .gradient {
            float: left;
            width: 100px;
            height: 400px;
            shape-outside: repeating-linear-gradient(transparent 0px, transparent 100px, white 200px);
            shape-image-threshold: 0.5;
        }
        #failure {
            position: absolute;
            top: 70px;
            width: 200px;
            font-size: 0px;
            z-index: -1;
        }
        #failure > .square {
            background-color: red;
        }
        #failure > .right {
            margin-left: 100px;
        }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “shape-image-threshold”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
