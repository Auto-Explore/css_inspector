# css/CSS2/tables/border-collapse-applies-to-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-collapse-applies-to-001.xht"
}
```

## style[0]

```css

            div
            {
                border-collapse: collapse;
                color: white;
                display: inline;
                font-size: 4em;
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
