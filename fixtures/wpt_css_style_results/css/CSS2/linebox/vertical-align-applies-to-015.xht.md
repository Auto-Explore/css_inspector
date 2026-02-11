# css/CSS2/linebox/vertical-align-applies-to-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/vertical-align-applies-to-015.xht"
}
```

## style[0]

```css

            div
            {
                color: blue;
                font: 20px/1em Ahem;
            }
            #table
            {
                display: table;
            }
            #caption
            {
                background: orange;
                display: table-caption;
                height: 1in;
                vertical-align: bottom;
                width: 1in;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                display: table-cell;
                height: 1in;
                width: 1in;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
