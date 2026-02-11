# css/css-tables/absolute-tables-001.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/absolute-tables-001.html"
}
```

## style[0]

```css

    main div {
        position: relative;
        border: 5px solid black;
        height: 60px;
        width: 60px;
        padding: 5px 7px 11px 13px;
        margin: 10px;
    }

    .tbl {
        display: table;
        background-color: skyblue;
        position: absolute;
        width: 50%;
        height: 50%;
    }

    .cell {
        display: table-cell;
        outline: 1px dashed blue;
    }

    .topleft { left: 0; top: 0; }
    .topright { right: 0; top: 0; }
    .bottomright { right: 0; bottom: 0; }
    .bottomleft { left: 0; bottom: 0; }

    .vertical  { writing-mode: vertical-lr; }

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
