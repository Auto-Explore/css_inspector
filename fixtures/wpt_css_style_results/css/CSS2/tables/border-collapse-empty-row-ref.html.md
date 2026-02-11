# css/CSS2/tables/border-collapse-empty-row-ref.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-collapse-empty-row-ref.html"
}
```

## style[0]

```css

table {
  display: inline-table;
  border-collapse: collapse;
}

td {
  border: 10px solid black;
  line-height: 0;
  padding: 0;
}

span {
  display: inline-block;
  width: 10px;
  height: 10px;
  background: grey;
}

.spacer-1 tr:not(:last-child) td {
  border-bottom: 12px solid black;
}

.spacer-2 tr:not(:last-child) td {
  border-bottom: 15px solid black;
}

.spacer-3 tr:not(:last-child) td {
  border-bottom: 20px solid black;
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
