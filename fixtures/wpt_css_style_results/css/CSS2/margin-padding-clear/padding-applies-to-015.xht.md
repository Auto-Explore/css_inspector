# css/CSS2/margin-padding-clear/padding-applies-to-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-applies-to-015.xht"
}
```

## style[0]

```css

            #table
            {
                display: table;
            }
            #test
            {
                border: 10px solid blue;
                display: table-caption;
                padding: 50px;
                width: 220px;
            }
            #test div
            {
                border: 10px solid orange;
                height: 200px;
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
