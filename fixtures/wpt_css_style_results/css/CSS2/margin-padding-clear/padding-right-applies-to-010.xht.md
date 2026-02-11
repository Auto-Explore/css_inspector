# css/CSS2/margin-padding-clear/padding-right-applies-to-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-right-applies-to-010.xht"
}
```

## style[0]

```css

            div
            {
                height: 200px;
            }
            #div1
            {
                border-right: 10px solid blue;
                display: list-item;
                margin-left: 2em;
                padding-right: 50px;
                width: 10px;
            }
            div div
            {
                border-right: 10px solid orange;
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
