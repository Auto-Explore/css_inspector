# css/css-shapes/shape-outside/shape-image/gradients/shape-outside-linear-gradient-002.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-image/gradients/shape-outside-linear-gradient-002.html"
}
```

## style[0]

```css

        .container {
            width: 200px;
            height: 200px;
            background-color: red;
            font-family: Ahem;
            font-size: 50px;
            line-height: 1;
        }
        #test {
            color: green;
        }
        #gradient {
            float: right;
            width: 200px;
            height: 200px;
            background: linear-gradient(to left, green 50%, transparent 50%);
            shape-outside: linear-gradient(to left, rgba(51, 51, 51, 1) 0%, rgba(51, 51, 51, 0.75) 50%, rgba(51, 51, 51, 0.25) 50%, transparent 100%);
            shape-image-threshold: 0.5;
        }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
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
