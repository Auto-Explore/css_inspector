# css/CSS2/margin-padding-clear/margin-right-applies-to-013.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-right-applies-to-013.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border-right: 10px solid orange;
				float: left;
            }
            #table
            {
                border-right: 10px solid blue;
                display: table;
                margin-right: 50px;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                display: table-cell;
                height: 200px;
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
