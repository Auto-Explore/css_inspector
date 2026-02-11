# css/CSS2/margin-padding-clear/margin-right-applies-to-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-right-applies-to-008.xht"
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
                border-right: 5px solid blue;
            }
            div div
            {
                border-right: 5px solid orange;
                margin-right: 50px;
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
