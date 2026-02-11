# css/css-tables/background-clip-001.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/background-clip-001.html"
}
```

## style[0]

```css

table {
  border-collapse: collapse;
  /* The properties after the blank line are not the behavior under test. */

  background: red;
}

td {
  background-clip: content-box;
  border: 30px solid green;

  background-color: green;
  padding: 0px;
  line-height: 0px;
}

.cell-content {
  display: inline-block;

  height: 40px;
  width: 40px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
