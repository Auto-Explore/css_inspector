# css/css-transforms/transform-origin-013.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-origin-013.html"
}
```

## style[0]

```css

    div {
      width: 100px;
      height: 100px;
    }
    .gradient{
      background-image: linear-gradient(orange 50%, fuchsia 50%);
      transform: rotate(90deg);
      transform-origin: top right;
    }

    .red {
      width: 98px;
      height: 98px;
      background: red;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
