# css/CSS2/positioning/absolute-replaced-width-011.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-width-011.xht"
}
```

## style[0]

```css

            #div1
            {
                border: solid black;
                direction: ltr;
                height: 3in;
                width: 3in;
            }
            #div2
            {
                height: 110px;
                position: relative;
                width: 3in;
            }
            #div3
            {
                background: orange;
                height: 1in;
                width: 200px;
            }
            svg
            {
                left: auto;
                position: absolute;
                right: auto;
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
