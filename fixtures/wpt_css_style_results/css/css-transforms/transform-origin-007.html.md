# css/css-transforms/transform-origin-007.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-origin-007.html"
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
    margin: 0px 50px 100px 50px;
    transform: rotate(180deg);
    transform-origin: 50% bottom;
    width: 50px;
  }
  #ref {
    background-color: green;
    height: 50px;
    margin: 0px 50px 100px 50px;
    transform: rotate(180deg);
    transform-origin: 50% 100%;
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
