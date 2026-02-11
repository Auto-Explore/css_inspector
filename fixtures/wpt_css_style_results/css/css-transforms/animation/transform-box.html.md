# css/css-transforms/animation/transform-box.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/transform-box.html"
}
```

## style[0]

```css

  .block {
    position: absolute;
    border: 20px solid black;
    width: 100px;
    height: 100px;
    left: 100px;
    top: 100px;
  }

  #transformBoxTarget {
    transform: rotateZ(90deg);
    transform-origin: 0% 100%;
    transform-box: border-box;
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
