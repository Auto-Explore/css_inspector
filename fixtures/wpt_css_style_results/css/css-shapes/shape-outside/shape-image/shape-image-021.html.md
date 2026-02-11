# css/css-shapes/shape-outside/shape-image/shape-image-021.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-image/shape-image-021.html"
}
```

## style[0]

```css

        #container {
          position: relative;
          width: 200px;
          font-family: Ahem;
          font-size: 40px;
          text-align: right;
          color: green;
        }
        #image {
          float: right;
          margin-left: 100px;
          shape-outside: url("support/right-half-rectangle.png"); /* size: 100x100, only the right 50x100 half is opaque */
          shape-margin: 10%; /* overall shape is 120x90 rectangle with corner radii = 20px */
          shape-image-threshold: 0.5;
        }
        #failure {
          position: absolute;
          top: 0px;
          left: 90px; /* container.width - shape-outside+margin.width - font-size */
          width: 200px;
          text-align: left;
          color: red;
          z-index: -1;
        }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “shape-margin”.",
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
