# css/CSS2/linebox/vertical-align-applies-to-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/vertical-align-applies-to-008.xht"
}
```

## style[0]

```css

            div
            {
                background: orange;
            }
            div div
            {
                display: inline;
            }
            #div1
            {
                color: blue;
                font: 20px/1em Ahem;
                vertical-align: bottom;
            }
            #div2
            {
                color: orange;
                font: 100px/1em Ahem;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
