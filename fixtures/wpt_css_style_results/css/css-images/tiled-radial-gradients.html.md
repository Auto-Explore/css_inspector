# css/css-images/tiled-radial-gradients.html

```json
{
  "format_version": 3,
  "file": "css/css-images/tiled-radial-gradients.html"
}
```

## style[0]

```css

            body {
                margin: 0px;
            }
            #gradient {
                position: absolute;
                width: 600px;
                height: 200px;
                left: 0px;
                margin: 0px;
                background-color: aquamarine;
                background-image: radial-gradient(closest-side, red 40%, transparent 40%);
                background-size: 300px 200px;
                background-position: 80px 0px;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
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
