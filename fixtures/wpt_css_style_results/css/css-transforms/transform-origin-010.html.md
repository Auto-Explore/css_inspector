# css/css-transforms/transform-origin-010.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-origin-010.html"
}
```

## style[0]

```css

  .div1 {
    height: 150px;
    position: absolute;
    width: 150px;
  }
  .div2 {
    background-color: gray;
  }
  #test {
    background-color: red;
    height: 50px;
    margin: 100px 0px 0px 100px;
    transform: rotate(180deg);
    transform-origin: left 0%;
    width: 50px;
  }
  #ref {
    background-color: green;
    height: 50px;
    margin: 100px 0px 0px 100px;
    transform: rotate(180deg);
    transform-origin: 0% 0%;
    width: 50px;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
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
