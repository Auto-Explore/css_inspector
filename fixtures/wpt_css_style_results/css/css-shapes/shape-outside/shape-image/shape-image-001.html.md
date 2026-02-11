# css/css-shapes/shape-outside/shape-image/shape-image-001.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-image/shape-image-001.html"
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
            width: 150px;
            color: rgb(0, 100, 0);
            background-color: black;
        }
        #image {
            float: left;
            width: 100px;
            height: 100px;
            shape-outside: url("support/left-half-rectangle-70.png");
        }
        #failure {
            width: 50px;
            height: 100px;
            background-color: red;
            position: absolute;
            top: 0px;
            right: 0px;
            z-index: -1;
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
