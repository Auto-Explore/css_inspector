# css/css-transitions/animations/move-after-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/animations/move-after-transition.html"
}
```

## style[0]

```css

    #container {
      position: relative;
      width: 400px;
      height: 100px;
      border: 1px solid  black;
    }
    #box {
      position: absolute;
      width: 100px;
      height: 100px;
      background-color: green;
      transform-style: preserve-3d;
      transition: transform 300ms linear;
      transform: translateX(0);
    }
    #container.moved #box {
      transform: translateX(300px);
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
