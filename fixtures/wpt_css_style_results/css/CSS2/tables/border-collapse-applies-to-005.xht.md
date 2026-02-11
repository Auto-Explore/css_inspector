# css/CSS2/tables/border-collapse-applies-to-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-collapse-applies-to-005.xht"
}
```

## style[0]

```css

            div
            {
                border-collapse: collapse;
                display: inline-block;
                height: 100px;
                width: 100px;
            }
            #left
            {
                border-right: 10px solid blue;
            }
            #right
            {
                border-left: 10px dotted orange;
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
