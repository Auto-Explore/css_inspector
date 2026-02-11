# css/css-shapes/shape-outside/shape-image/shape-image-019.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-image/shape-image-019.html"
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
            margin-top: 10px;
            margin-bottom: 100px;
            /* 20x20 solid blue PNG */
            shape-outside: url("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUCAYAAACNiR0NAAAAK0lEQVQ4T2NkYPj/n4GKgHHUQIpDczQMKQ5ChtEwHA1DMkJgNNmQEWhoWgAkKift4VBlVgAAAABJRU5ErkJggg==");
            shape-margin: 10px;
        }
        #failure {
            position: absolute;
            top: 0px;
            left: 130px; /* container.width - shape-outside+margin.width - font-size */
            width: 200px;
            text-align: left;
            color: red;
            z-index: -1;
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
      "message": "Unknown property “shape-margin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
