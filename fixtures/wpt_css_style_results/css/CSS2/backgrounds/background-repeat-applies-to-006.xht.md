# css/CSS2/backgrounds/background-repeat-applies-to-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-repeat-applies-to-006.xht"
}
```

## style[0]

```css

            #test
            {
                background-color: red;
                background-image: url('support/green15x15.png');
                background-repeat: repeat;
                display: table-column;
                width: 100px;
            }
            #table
            {
                display: table;
                table-layout: fixed;
                width: 200px;
            }
            .column
            {
                display: table-column;
            }
            .row
            {
                display: table-row;
            }
            .cell
            {
                color: green;
                display: table-cell;
                height: 50px;
            }

            div.cell + div.cell {color: white;}
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
