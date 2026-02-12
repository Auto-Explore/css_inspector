# css/css-transforms/perspective-origin-004.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/perspective-origin-004.html"
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
    perspective-origin: 50% top;
  }
  #redSquare {
    background-color: red;
    transform: translateZ(0px);
  }
  #ref {
    perspective: 2px;
    perspective-origin: 50% 0%;
  }
  #greenSquare {
    background-color: green;
    height: 50px;
    left: 25px;
    transform: translateZ(1px);
    width: 50px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
