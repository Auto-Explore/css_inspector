# css/CSS2/floats-clear/float-replaced-height-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/float-replaced-height-002.xht"
}
```

## style[0]

```css

            div
            {
                line-height: 0;
                position: relative;
            }
            div div
            {
                background: orange;
                height: 15px;
                left: 15px;
                position: absolute;
                top: 0;
                width: 15px;
            }
            img
            {
                height: auto;
                width: auto;
            }
            div, img
            {
                float: left;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
