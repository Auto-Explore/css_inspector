# css/CSS2/linebox/line-height-normal-recommendation-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/line-height-normal-recommendation-001.xht"
}
```

## style[0]

```css

            div
            {
                font: 1in Ahem;
            }
            #div1
            {
                position: relative;
            }
            div div
            {
                position: absolute;
                top: 0;
            }
            #div2
            {
                height: 1em;
                left: 0;
                width: 1in;
            }
            #div3
            {
                background: blue;
                display: block;
                left: 1in;
                position: absolute;
                width: 1in;
            }
            #div4
            {
                height: 1.2em;
                left: 2in;
                width: 1in;
            }
            #div2, #div4
            {
                background: orange;
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
