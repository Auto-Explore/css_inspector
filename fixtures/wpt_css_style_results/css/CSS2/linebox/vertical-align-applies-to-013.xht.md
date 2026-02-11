# css/CSS2/linebox/vertical-align-applies-to-013.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/vertical-align-applies-to-013.xht"
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
                table-layout: fixed;
                vertical-align: bottom;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                background: orange;
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
