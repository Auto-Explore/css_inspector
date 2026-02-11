# css/CSS2/margin-padding-clear/margin-bottom-applies-to-013.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-bottom-applies-to-013.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border-bottom: 10px solid blue;
            }
            #table
            {
                border-bottom: 10px solid orange;
                display: table;
                table-layout: fixed;
                margin-bottom: 50px;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                display: table-cell;
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
