# css/CSS2/tables/table-margin-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/table-margin-004.xht"
}
```

## style[0]

```css

            div#wrapper
            {
                border: blue solid medium;
                height: 180px;
                /*
                  20px (margin-top) + 20px (content) + max(20px, 20px) 1st div table
                + 20px (content) + max(20px, 20px)  2nd div table
                + 20px (content) + max(20px, 20px)  1st p table
                + 20px (content) + 20px 2nd p table
                ========================================================
                180px
                */
            }

            .table
            {
                border-spacing: 0;
                display: table;
                font: 20px/1 Ahem;
                height: 1em;
                margin: 1em;
            }

            div.table-row
            {
                display: table-row;
            }

            div.cell
            {
                display: table-cell;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
