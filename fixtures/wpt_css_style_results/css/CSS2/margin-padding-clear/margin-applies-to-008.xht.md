# css/CSS2/margin-padding-clear/margin-applies-to-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-applies-to-008.xht"
}
```

## style[0]

```css

            div
            {
                display: inline;
            }
            #div1
            {
                border-left: 5px solid blue;
                border-right: 5px solid blue;
            }
            div div
            {
                border-left: 5px solid orange;
                border-right: 5px solid orange;
                margin: 50px;
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
