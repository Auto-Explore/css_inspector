# css/CSS2/margin-padding-clear/padding-top-applies-to-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-top-applies-to-004.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border-top: 10px solid blue;
            }
            #table
            {
                display: table;
            }
            #row
            {
                display: table-row;
                padding-top: 50px;
            }
            #cell
            {
                display: table-cell;
            }
            #cell div
            {
                border-top: 10px solid orange;
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
