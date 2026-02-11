# css/CSS2/normal-flow/inline-non-replaced-height-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/inline-non-replaced-height-002.xht"
}
```

## style[0]

```css

            #div1
            {
                margin-top: 41px;
                position: relative;
            }
            span
            {
                background: blue;
                color: blue;
                font: 100px Ahem;
                line-height: 150px;
                padding: 50px 0;
            }
            div div
            {
                background: orange;
                height: 200px;
                left: 100px;
                position: absolute;
                top: -25px; /* correspond to minus top-half-leading */
                width: 100px;
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
