# css/CSS2/zindex/z-index-applies-to-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/zindex/z-index-applies-to-004.xht"
}
```

## style[0]

```css

            #table
            {
                display: table;
                table-layout: fixed;
            }
            #row
            {
                background: green;
                display: table-row;
                z-index: 1;
            }
            #cell
            {
                display: table-cell;
            }
            #zindex
            {
                background: yellow;
                top: -1in;
            }
            #row, #zindex
            {
                position: relative;
            }
            #cell, #zindex
            {
                height: 1in;
                width: 1in;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
