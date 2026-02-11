# css/css-transforms/css-rotate-2d-3d-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/css-rotate-2d-3d-001.html"
}
```

## style[0]

```css

        .greenSquare {
            position: absolute;
            top: 50px;
            left: 100px;
            width: 100px;
            height: 200px;
            background: green;
                transform: rotate(90deg) rotate3d(1, 0, 0, 60deg);
         }
        .redSquare {
            position: absolute;
            top: 101px;
            left: 101px;
            width: 98px;
            height: 98px;
            background: red;
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
