# css/CSS2/tables/border-collapse-applies-to-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-collapse-applies-to-010.xht"
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
                border-collapse: collapse;
                display: table-footer-group;
            }
            #tr
            {
                display: table-row;
            }
            #left, #right
            {
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
