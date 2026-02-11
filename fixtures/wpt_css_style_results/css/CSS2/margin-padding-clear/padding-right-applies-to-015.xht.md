# css/CSS2/margin-padding-clear/padding-right-applies-to-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-right-applies-to-015.xht"
}
```

## style[0]

```css

            #test
            {
                border-right: 10px solid orange;
                display: table-caption;
                padding-right: 50px;
            }
            #test div
            {
                border-right: 10px solid blue;
                height: 200px;
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
