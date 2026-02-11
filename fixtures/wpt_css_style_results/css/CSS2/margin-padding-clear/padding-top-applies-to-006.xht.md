# css/CSS2/margin-padding-clear/padding-top-applies-to-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-top-applies-to-006.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border-top: 10px solid blue;
            }
            #test
            {
                display: table-column;
                padding-top: 50px;
            }
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
