# css/css-transforms/ttwf-css-3d-polygon-cycle.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/ttwf-css-3d-polygon-cycle.html"
}
```

## style[0]

```css

    #container {
      position: absolute;
      top: 100px;
      left: 100px;
      transform-style: preserve-3d;
    }
    .rect {
      position: absolute;
    }
    #red {
      background-color: red;
      width: 200px;
      height: 50px;
      transform: rotateY(20deg);
    }
    #green {
      background-color: green;
      width: 50px;
      height: 200px;
      transform: rotateX(20deg);
    }
    #blue {
      background-color: blue;
      width: 50px;
      height: 200px;
      transform: rotate(45deg) translate(50px, -50px) rotateX(-20deg);
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
