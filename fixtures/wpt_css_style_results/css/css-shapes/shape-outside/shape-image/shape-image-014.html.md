# css/css-shapes/shape-outside/shape-image/shape-image-014.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-image/shape-image-014.html"
}
```

## style[0]

```css

        .container {
            position: relative;
            font-family: Ahem;
            font-size: 50px;
            line-height: 50px;
            color: rgb(0, 100, 0);
            background-color: red;
        }
        #test {
            width: 100px;
            height: 100px;
            text-align: right;
        }
        #image {
            float: right;
            margin-left: 10px;
            shape-outside: url("support/right-half-rectangle.svg");
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
