# css/css-backgrounds/reference/subpixel-repeat-no-repeat-mix-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/reference/subpixel-repeat-no-repeat-mix-ref.html"
}
```

## style[0]

```css

    div.x {
      position: absolute;
      left: 20px;
      top: 10px;
      width: 117px;
      height: 10px;
      background-image: url("../resources/green-right.png");
      background-position-x: right;
      background-repeat: repeat-y;
      background-size: 117px 1px;
    }
    div.y {
      position: absolute;
      left: 10px;
      top: 20px;
      width: 10px;
      height: 117px;
      background-image: url("../resources/green-bottom.png");
      background-position-y: bottom;
      background-repeat: repeat-x;
      background-size: 1px 117px;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
