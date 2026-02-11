# css/CSS2/margin-padding-clear/padding-right-applies-to-012.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-right-applies-to-012.xht"
}
```

## style[0]

```css

            div
            {
                height: 200px;
                display: inline-block;
            }
            #div1
            {
                border-right: 10px solid orange;
                padding-right: 50px;
            }
            div div
            {
                border-right: 10px solid blue;
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
