# css/CSS2/floats-clear/float-non-replaced-height-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/float-non-replaced-height-001.xht"
}
```

## style[0]

```css

            #div1
            {
                position: relative;
            }
            #div2
            {
                background: blue;
                float: left;
            }
            #div2 div, #div3
            {
                height: 1in;
                width: 1in;
            }
            #div3
            {
                background: orange;
                left: 1in;
                position: absolute;
                top: 0;
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
