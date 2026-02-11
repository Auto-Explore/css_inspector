# css/css-transforms/preserve3d-backface-hit-test-1.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/preserve3d-backface-hit-test-1.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  .container {
    position: absolute;
    top: 50px;
    left: 50px;
    perspective: 600px;
    width: 200px;
    height: 200px;
  }

  .rotatetoback {
    width: 100%;
    height: 100%;
    transform-style: preserve-3d;
    transform: rotateY(-180deg);
  }

  .hideback {
    position: absolute;
    width: 100%;
    height: 100%;
    backface-visibility: hidden;
  }

  .rotatetofront {
    transform: rotateY(180deg);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
