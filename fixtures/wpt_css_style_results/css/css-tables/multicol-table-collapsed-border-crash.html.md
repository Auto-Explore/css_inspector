# css/css-tables/multicol-table-collapsed-border-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/multicol-table-collapsed-border-crash.html"
}
```

## style[0]

```css

  html, body, span {
    column-count: 2;
  }
  span {
    display: table;
    border-collapse: collapse;
    border: 1px solid black;
  }
  a {
    vertical-align: 10000000000000000em;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
