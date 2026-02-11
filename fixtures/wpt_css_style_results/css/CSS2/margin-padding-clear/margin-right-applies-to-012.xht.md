# css/CSS2/margin-padding-clear/margin-right-applies-to-012.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-right-applies-to-012.xht"
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
                border-right: 10px solid orange;
                float: left;
            }
            div div
            {
                border-right: 10px solid blue;
                display: inline-block;
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
