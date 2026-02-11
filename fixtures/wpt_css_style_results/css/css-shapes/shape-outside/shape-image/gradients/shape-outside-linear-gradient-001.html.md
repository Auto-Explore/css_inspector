# css/css-shapes/shape-outside/shape-image/gradients/shape-outside-linear-gradient-001.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-image/gradients/shape-outside-linear-gradient-001.html"
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
            float: left;
            width: 200px;
            height: 200px;
            background: linear-gradient(to right, green 50%, transparent 50%);
            shape-outside: linear-gradient(to right, rgba(51, 51, 51, 1) 0%, rgba(51, 51, 51, 0.5) 50%, transparent 50%);
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
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
