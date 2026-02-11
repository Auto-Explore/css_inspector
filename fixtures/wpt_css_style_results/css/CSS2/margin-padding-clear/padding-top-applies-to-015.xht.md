# css/CSS2/margin-padding-clear/padding-top-applies-to-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-top-applies-to-015.xht"
}
```

## style[0]

```css

            #test
            {
                border-top: 10px solid blue;
                display: table-caption;
                padding-top: 50px;
            }
            #test div
            {
                border-top: 10px solid orange;
                width: 200px;
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
