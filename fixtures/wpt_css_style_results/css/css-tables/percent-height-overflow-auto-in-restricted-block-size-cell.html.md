# css/css-tables/percent-height-overflow-auto-in-restricted-block-size-cell.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/percent-height-overflow-auto-in-restricted-block-size-cell.html"
}
```

## style[0]

```css

  .table { display:table; height:100px; background:pink; }
  .cell { overflow:auto; width:100px; height:100%; background:blue; }
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
