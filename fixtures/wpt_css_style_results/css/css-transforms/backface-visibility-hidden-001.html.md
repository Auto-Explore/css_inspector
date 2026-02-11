# css/css-transforms/backface-visibility-hidden-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/backface-visibility-hidden-001.html"
}
```

## style[0]

```css

        .greenSquare {
            position: absolute;
            top: 50px;
            left: 50px;
            width: 100px;
            height: 100px;
            background: green;
            z-index: 2;
        }

        .redSquare {
            position: absolute;
            top: 50px;
            left: 50px;
            width: 100px;
            height: 100px;
            background: red;
            z-index: 1;
            transform: rotateY(180deg);
        }

        .face {
            backface-visibility: hidden;
        }

        .card {
            transform-style: preserve-3d;
        }

        .container {
            width: 200px;
            height: 200px;
            perspective: 1000px;
            transform: rotateY(45deg);
        }

    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
