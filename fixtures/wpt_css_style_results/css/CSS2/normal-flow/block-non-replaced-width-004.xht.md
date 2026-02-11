# css/CSS2/normal-flow/block-non-replaced-width-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/block-non-replaced-width-004.xht"
}
```

## style[0]

```css

            div
            {
                direction: rtl;
                position: relative;
            }
            #div1
            {
                background: orange;
                width: 100px;
            }
            #div1 div
            {
                background-color: orange;
                border-left: 10px solid orange;
                border-right: 10px solid orange;
                height: 30px;
                margin-left: 10px;
                margin-right: 10px;
                padding-left: 10px;
                padding-right: 10px;
                width: 100px;
            }

			/*
			In this test, the used margin-left should be -50px so
			that the equation remains balanced.
			*/

            #div2
            {
                background-color: blue;
                height: 30px;
                position: absolute;
                width: 150px;
            }
        
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
