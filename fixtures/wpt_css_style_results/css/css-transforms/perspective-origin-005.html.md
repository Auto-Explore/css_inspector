# css/css-transforms/perspective-origin-005.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/perspective-origin-005.html"
}
```

## style[0]

```css

  div {
    height: 100px;
    position: absolute;
    width: 100px;
  }
  #test {
    perspective: 2px;
    perspective-origin: left 50%;
  }
  #redSquare {
    background-color: red;
    transform: translateZ(0px);
  }
  #ref {
    perspective: 2px;
    perspective-origin: 0% 50%;
  }
  #greenSquare {
    background-color: green;
    height: 50px;
    top: 25px;
    transform: translateZ(1px);
    width: 50px;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “perspective-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “perspective-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
