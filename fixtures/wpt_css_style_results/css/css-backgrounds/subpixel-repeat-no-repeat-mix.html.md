# css/css-backgrounds/subpixel-repeat-no-repeat-mix.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/subpixel-repeat-no-repeat-mix.html"
}
```

## style[0]

```css

    div.x {
      position: absolute;
      left: 20px;
      top: 10px;
      width: 116.8px;
      height: 10px;
      background-image: url("resources/green-right.png");
      background-position-x: right;
      background-repeat: repeat-y;
      background-size: 116.8px 0.8px;
    }
    div.y {
      position: absolute;
      left: 10px;
      top: 20px;
      width: 10px;
      height: 116.8px;
      background-image: url("resources/green-bottom.png");
      background-position-y: bottom;
      background-repeat: repeat-x;
      background-size: 0.8px 116.8px;
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
