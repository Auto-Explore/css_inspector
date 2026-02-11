# css/css-grid/grid-definition/grid-auto-repeat-multiple-values-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/grid-auto-repeat-multiple-values-004-ref.html"
}
```

## style[0]

```css

        .columns {
            border: 1px solid black;
            width: 5px;
            display: grid;
            grid-template-columns: [u v] 10px [w] 10px [x] 10px [y v] 10px [w] 10px [x] 10px [y z];
            grid-auto-rows: 5px;
            grid-column-gap: 3px;
            /* Does not fit a whole-number of repetitions */
            width: 94px;
        }
        div > div {
            background: blue;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
