# css/css-backgrounds/animations/background-color-animation-with-images-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-color-animation-with-images-ref.html"
}
```

## style[0]

```css

.container1 {
  width: 200px;
  height: 200px;
  background-image: url("../support/green.png"), url("../support/red.png");
  background-size: 100px 100px;
  background-repeat: no-repeat;
  background-color: rgb(0, 0, 199);
}
.container2 {
  width: 200px;
  height: 200px;
  background-image: url("../support/green.png"), url("../support/red.png");
  background-size: 100px 100px;
  background-repeat: no-repeat;
}
.container3 {
  width: 100px;
  height: 100px;
  background-image: url("../support/green.png"), url("../support/red.png");
}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
