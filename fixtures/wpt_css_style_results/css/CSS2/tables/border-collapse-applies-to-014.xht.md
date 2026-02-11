# css/CSS2/tables/border-collapse-applies-to-014.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-collapse-applies-to-014.xht"
}
```

## style[0]

```css

            #table
            {
                display: table;
            }
            #tr
            {
                display: table-row;
            }
            .test
            {
                border-collapse: collapse;
                display: table-cell;
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
