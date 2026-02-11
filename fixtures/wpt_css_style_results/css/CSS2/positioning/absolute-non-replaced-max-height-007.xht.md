# css/CSS2/positioning/absolute-non-replaced-max-height-007.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-max-height-007.xht"
}
```

## style[0]

```css

            #div1
            {
                font: 100px/1 Ahem;
                height: 400px;
                position: relative;
                width: 100px;
            }
            #div2
            {
                background: orange;
                height: 50px;
                position: relative;
                top: 50px;
                width: 100px;
            }
            #div3
            {
                background: blue;
                bottom: 300px;
                height: auto;
                margin-bottom: auto;
                margin-top: auto;
                max-height: 50px;
                position: absolute;
                top: auto;
                width: 50px;
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
