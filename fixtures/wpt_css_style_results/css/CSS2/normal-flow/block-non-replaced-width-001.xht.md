# css/CSS2/normal-flow/block-non-replaced-width-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/block-non-replaced-width-001.xht"
}
```

## style[0]

```css

            #div1
            {
                background-color: orange;
                border: 5px solid orange;
                display: inline-block;
            }
            div div
            {
                background-color: orange;
                border-left: 10px solid orange;
                border-right: 10px solid orange;
                height: 20px;
                margin-left: 10px;
                margin-right: 10px;
                padding-left: 10px;
                padding-right: 10px;
                width: 80px;
            }
            #d3
            {
                background-color: blue;
                height: 30px;
                position: absolute;
                top: 82px;
                width: 150px;
            }
        
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
