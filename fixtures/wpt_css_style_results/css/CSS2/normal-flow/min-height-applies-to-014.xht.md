# css/CSS2/normal-flow/min-height-applies-to-014.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/min-height-applies-to-014.xht"
}
```

## style[0]

```css

            #table
            {
                display: inline-table;
                table-layout: fixed;
                min-height: 1in;
            }
            #row
            {
                display: table-row;
                height: inherit; /* height of the row is based on the computed value for height of the table which is based on the min-height property */
            }
            #cell
            {
                background: black;
                display: table-cell;
                height: inherit; /* height of the cell is based on the computed value for height of the table which is based on the min-height property */
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
