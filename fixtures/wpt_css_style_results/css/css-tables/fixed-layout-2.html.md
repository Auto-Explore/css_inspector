# css/css-tables/fixed-layout-2.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/fixed-layout-2.html"
}
```

## style[0]

```css

.wrapper {
  width: 0;
}
x-table {
  table-layout: fixed;
  border-spacing: 0px;
}
x-td:first-child {
  padding: 0;
  background: cyan;
  width: 50px;
  height: 50px;
}
x-td + x-td {
  padding: 0;
  height: 50px;
}
x-td > div {
  width: 100px;
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
