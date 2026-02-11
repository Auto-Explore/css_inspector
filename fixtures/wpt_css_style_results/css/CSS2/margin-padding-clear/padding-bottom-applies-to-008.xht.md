# css/CSS2/margin-padding-clear/padding-bottom-applies-to-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-bottom-applies-to-008.xht"
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
                border-bottom: 5px solid blue;
                padding-bottom: 50px;
            }
            div div
            {
                border-bottom: 5px solid orange;
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
