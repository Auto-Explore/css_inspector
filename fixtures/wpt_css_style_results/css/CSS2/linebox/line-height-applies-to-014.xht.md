# css/CSS2/linebox/line-height-applies-to-014.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/line-height-applies-to-014.xht"
}
```

## style[0]

```css

            #div1
            {
                position: relative;
            }
            #div2
            {
                background: orange;
                height: 2in;
                left: 1in;
                position: absolute;
                top: 0;
                width: 1in;
            }
            #table
            {
                display: inline-table;
                table-layout: fixed;
                line-height: 2in;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                background: blue;
                display: table-cell;
                width: 1in;
            }
            .box
            {
                background: black;
                height: 1em;
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
