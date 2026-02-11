# css/CSS2/margin-padding-clear/margin-applies-to-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-applies-to-004.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border: 10px solid blue;
                position: absolute;
            }
            #test
            {
                display: table-row;
                margin: 50px;
            }
            #table
            {
                display: table;
            }
            #cell
            {
                border: 10px solid orange;
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
