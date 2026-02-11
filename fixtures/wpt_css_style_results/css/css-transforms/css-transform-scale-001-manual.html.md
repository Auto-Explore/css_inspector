# css/css-transforms/css-transform-scale-001-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/css-transform-scale-001-manual.html"
}
```

## style[0]

```css

        .container {
            position: absolute;
        }
        .greenSquare {
            position: absolute;
            top: 0;
            left: 0;
            width: 100px;
            height: 100px;
            background-color: green;
         }
        .greenSquare:hover {
            transform-origin: 0 0;
            transform: scale(2);
        }

         /* This div should only be visible if the test fails */
        .redSquare {
            position: absolute;
            /* It is approximately the same size as the test div, but with a 1px margin */
            width: 98px;
            height: 98px;
            background-color: red;
            left: 100px;
            top :100px;
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
