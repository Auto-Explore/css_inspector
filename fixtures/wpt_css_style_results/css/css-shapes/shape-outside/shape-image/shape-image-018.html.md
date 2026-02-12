# css/css-shapes/shape-outside/shape-image/shape-image-018.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-image/shape-image-018.html"
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
            shape-outside: url("support/left-half-rectangle.jpg"); /* size: 100x100, no alpha channel */
            shape-margin: 10%; /* overall shape is 120x120 rectangle with corner radii = 10px */
        }
        #failure {
            position: absolute;
            top: 0px;
            left: 40px; /* container.width - shape-outside+margin.width - font-size */
            width: 200px;
            text-align: left;
            color: red;
            z-index: -1;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
