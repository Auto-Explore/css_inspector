# css/css-shapes/shape-outside/shape-image/shape-image-024.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-image/shape-image-024.html"
}
```

## style[0]

```css

        body {
            margin: 0;
        }
        .container {
          left: 10px;
          font: 100px/1 Ahem;
        }
        #test {
            width: 211px;
            color: rgb(0,100,0);
        }
        #image {
            float: left;
            width: 200px;
            height: 200px;
            shape-outside: url("support/left-half-rectangle.png");
            shape-margin: 10px;
        }
        .blue {
            width: 2px;
            height: 200px;
            background-color: blue;
        }
        .left-line { left: 115px; }
        .right-line { left: 230px; }

        .failure {
            left: 120px;
            width: 100px;
            height: 200px;
            background-color: red;
            z-index: -1;
        }
        .container, .blue, .failure {
            position: absolute;
            top: 70px;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
