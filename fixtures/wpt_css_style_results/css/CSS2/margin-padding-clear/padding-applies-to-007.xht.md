# css/CSS2/margin-padding-clear/padding-applies-to-007.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-applies-to-007.xht"
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
                border: 10px solid blue;
                display: table-cell;
                padding: 50px;
            }
            #cell div
            {
                border: 10px solid orange;
                height: 200px;
                width: 200px;
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
