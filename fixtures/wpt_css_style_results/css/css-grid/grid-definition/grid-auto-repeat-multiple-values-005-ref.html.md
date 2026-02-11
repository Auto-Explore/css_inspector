# css/css-grid/grid-definition/grid-auto-repeat-multiple-values-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/grid-auto-repeat-multiple-values-005-ref.html"
}
```

## style[0]

```css

        .rows {
            border: 1px solid black;
            display: inline-grid;
            grid-auto-flow: column;
            grid-template-rows: [u v] 10px [w] 10px [x] 10px [y v] 10px [w] 10px [x] 10px [y z];
            grid-auto-columns: 5px;
            grid-row-gap: 3px;
            width: min-content;
            /* Does not fit a whole-number of repetitions */
            height: 94px;
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
