# css/css-transforms/transformed-rotateX-3.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transformed-rotateX-3.html"
}
```

## style[0]

```css

    div {
      height: 150px;
      width: 150px;
    }
    .container {
      background-color: red;
    }
    .transformed {
      transform-origin: top left;
      transform: rotateX(180deg);
      background-color: green;
      position: relative;
      top: 150px;
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
