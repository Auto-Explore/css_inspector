# css/css-grid/implicit-grids/grid-support-grid-auto-columns-rows-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/implicit-grids/grid-support-grid-auto-columns-rows-001.html"
}
```

## style[0]

```css

         #grid {
             display: grid;
             grid-auto-columns: 30px;
             grid-auto-rows: 30px;
         }
         #first-column-first-row {
             grid-column: 1;
             grid-row: 1;
             background-color: purple;
         }
         #third-column-first-and-second-rows {
             grid-column: 3;
             grid-row: 1 / span 2;
             background-color: orange
         }
         #first-and-second-columns-second-row {
             grid-column: 1 / span 2;
             grid-row: 2;
             background-color: blue;
         }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
