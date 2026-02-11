# css/css-conditional/container-queries/column-spanner-in-container.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/column-spanner-in-container.html"
}
```

## style[0]

```css

  #multicol {
    container-type: inline-size;
    width: 600px;
    columns: 2;
    column-gap: 0;
    height: 200px;
  }
  #spanner { height: 100px; }
  @container (width = 600px) {
    #spanner {
      column-span: all;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
