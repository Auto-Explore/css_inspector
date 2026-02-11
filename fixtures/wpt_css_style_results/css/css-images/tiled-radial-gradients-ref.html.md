# css/css-images/tiled-radial-gradients-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/tiled-radial-gradients-ref.html"
}
```

## style[0]

```css

            body {
                margin: 0px;
            }

            #outer {
                position: absolute;
                width: 600px;
                height: 200px;
                background-color: aquamarine;
            }

            #left, #right {
                position: absolute;
                width: 300px;
                height: 200px;
                background-image: radial-gradient(closest-side, red 40%, transparent 40%)

            }
            #left {
                left: 80px;
            }

            #right {
                left: 380px;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
