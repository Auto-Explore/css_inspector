# css/CSS2/margin-padding-clear/padding-left-applies-to-007.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-left-applies-to-007.xht"
}
```

## style[0]

```css

            #table
            {
                display: table;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                border-left: 10px solid blue;
                display: table-cell;
                padding-left: 50px;
            }
            #cell div
            {
                border-left: 10px solid orange;
                height: 200px;
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
