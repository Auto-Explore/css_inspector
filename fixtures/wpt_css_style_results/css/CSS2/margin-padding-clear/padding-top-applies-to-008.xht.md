# css/CSS2/margin-padding-clear/padding-top-applies-to-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-top-applies-to-008.xht"
}
```

## style[0]

```css

            p
            {
                margin-bottom: 100px;
            }
            div
            {
                display: inline;
            }
            #div1
            {
                border-top: 5px solid blue;
                padding-top: 50px;
            }
            div div
            {
                border-top: 5px solid orange;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
