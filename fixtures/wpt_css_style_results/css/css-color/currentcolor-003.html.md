# css/css-color/currentcolor-003.html

```json
{
  "format_version": 3,
  "file": "css/css-color/currentcolor-003.html"
}
```

## style[0]

```css

    div {
      color: red;
    }
    div::first-line {
      color: green;
    }
    br {
      line-height: 0;
    }
    span {
      box-decoration-break: clone;
      -webkit-box-decoration-break: clone;
      line-height: 50px;
      background-image: linear-gradient(currentcolor, currentcolor);
      background-color: currentColor;
      background-size: 100% 75%;
      background-repeat: no-repeat;
      border: 5px solid currentColor;
      outline: 5px solid currentColor;
      box-shadow: 111px 0 5px currentColor;
      text-shadow: 222px 0 2px currentColor;
      filter: drop-shadow(333px 0 0 currentColor);
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
