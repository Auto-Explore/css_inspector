# css/CSS2/margin-padding-clear/padding-bottom-applies-to-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-bottom-applies-to-001.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border-bottom: 10px solid blue;
            }
            #test
            {
                display: table-row-group;
                padding-bottom: 50px;
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
                border-bottom: 10px solid orange;
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
