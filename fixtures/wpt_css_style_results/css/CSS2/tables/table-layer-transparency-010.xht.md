# css/CSS2/tables/table-layer-transparency-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/table-layer-transparency-010.xht"
}
```

## style[0]

```css

            table
            {
                background: orange;
                border-collapse: separate;
                border-spacing: 0;
            }
            colgroup, col, tbody, tr
            {
                background: blue;
            }
            td
            {
                height: 2em;
                width: 2em;
            }
            #test
            {
                background: red;
                empty-cells: hide;
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
