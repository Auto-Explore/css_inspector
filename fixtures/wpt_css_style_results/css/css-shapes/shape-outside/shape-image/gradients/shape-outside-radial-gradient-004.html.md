# css/css-shapes/shape-outside/shape-image/gradients/shape-outside-radial-gradient-004.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-image/gradients/shape-outside-radial-gradient-004.html"
}
```

## style[0]

```css

        body { margin: 0; }
        .container {
            text-align: right;
            width: 200px;
            height: 200px;
            font-family: Ahem;
            font-size: 10px;
            line-height: 1;
        }
        #test {
            color: green;
        }
        #gradient {
            float: right;
            width: 100px;
            height: 100px;
            shape-outside: radial-gradient(circle, green 10px, rgba(0, 255, 0, 0.75) 50px, rgba(0, 255, 0, 0.25) 50px);
            shape-image-threshold: 0.5;
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
