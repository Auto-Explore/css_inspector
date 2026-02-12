# css/css-transforms/transformed-preserve-3d-1.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transformed-preserve-3d-1.html"
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
    height: 300px;
    position: relative;
    top: -150px;
    transform-origin: bottom left;
    transform-style: preserve-3d;
    transform: rotateX(60deg);
    background-color: green;
  }
  .child {
    transform-origin: top left;
    transform: rotateX(30deg);
    background-color: red;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
