# css/CSS2/tables/table-header-group-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/table-header-group-003.xht"
}
```

## style[0]

```css

            .table
            {
                display: table;
            }
            .caption
            {
                border: 1px solid black;
                background: orange;
                caption-side: top;
                color: orange;
                display: table-caption;
                height: 4em;
                width: 4em;
            }
            #thead
            {
                display: table-header-group;
                background: blue;
            }
            #tbody
            {
                display: table-row-group;
                background: orange;
            }
            .tr
            {
                display: table-row;
            }
            .td
            {
                display: table-cell;
                border: 1px solid black;
                height: 4em;
                width: 4em;
            }
        
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
