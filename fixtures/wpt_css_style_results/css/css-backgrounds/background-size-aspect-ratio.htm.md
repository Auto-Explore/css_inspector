# css/css-backgrounds/background-size-aspect-ratio.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-size-aspect-ratio.htm"
}
```

## style[0]

```css

            .reference
            {
                width: 1in;
                height: 1in;
                background: url("../support/cat.png") no-repeat;
                background-size: 80px 80px;
            }
            .test1
            {
                width: 400px;
                height: 150px;
                border: thick solid black;
                background-image: url("../support/cat.png");
                background-repeat: round no-repeat;
                background-size: 83px auto;
                margin-bottom: 10px;
            }
            .test2
            {
                width: 150px;
                height: 400px;
                border: thick solid black;
                background-image: url("../support/cat.png");
                background-repeat: no-repeat round;
                background-size: auto 83px;
            }
        
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-repeat”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-repeat”.",
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
