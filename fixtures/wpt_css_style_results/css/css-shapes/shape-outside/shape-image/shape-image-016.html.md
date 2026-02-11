# css/css-shapes/shape-outside/shape-image/shape-image-016.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-image/shape-image-016.html"
}
```

## style[0]

```css

        .container {
              position: relative;
              font-family: Ahem;
              font-size: 50px;
              line-height: 50px;
        }
        #test {
            width: 100px;
            color: rgb(0, 100, 0);
            background-color: red;
        }
        #image {
            float: right;
            shape-outside: url(support/right-half-rectangle-70.png);
            shape-image-threshold: 0.71;
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
