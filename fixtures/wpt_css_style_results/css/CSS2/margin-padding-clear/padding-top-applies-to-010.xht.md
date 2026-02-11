# css/CSS2/margin-padding-clear/padding-top-applies-to-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-top-applies-to-010.xht"
}
```

## style[0]

```css

            div
            {
                width: 200px;
            }
            #div1
            {
                border-top: 10px solid blue;
                display: list-item;
                margin-left: 2em;
                padding-top: 50px;
            }
            div div
            {
                border-top: 10px solid orange;
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
