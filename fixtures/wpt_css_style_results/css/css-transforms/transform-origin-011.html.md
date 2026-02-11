# css/css-transforms/transform-origin-011.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-origin-011.html"
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
    margin: 0px 100px 100px 0px;
    transform: rotate(180deg);
    transform-origin: right 100%;
    width: 50px;
  }
  #ref {
    background-color: green;
    height: 50px;
    margin: 0px 100px 100px 0px;
    transform: rotate(180deg);
    transform-origin: 100% 100%;
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
