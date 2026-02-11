# css/CSS2/margin-padding-clear/margin-right-092.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-right-092.xht"
}
```

## style[0]

```css

            div
            {
                direction: rtl;
                font: 20px/1 Ahem;
                height: 1in;
                width: 0;
            }
            #wrapper
            {
                border-right: 1ex solid red;
            }
            #div1
            {
                margin-right: +7.5ex;
            }
            #div2
            {
                border-right: 1ex solid black;
                margin-right: -8.5ex;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
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
