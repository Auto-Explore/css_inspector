# css/css-conditional/container-queries/container-inside-multicol-with-table.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-inside-multicol-with-table.html"
}
```

## style[0]

```css

  #multicol {
    columns: 2;
  }
  .container {
    container-type: inline-size;
    width: 100px;
  }
  @container (width = 100px) {
    #t1, #t2 { color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
