# css/css-grid/abspos/descendant-static-position-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/descendant-static-position-002-ref.html"
}
```

## style[0]

```css

.grid {
  position: relative;
  display: grid;
  grid: 40px / 40px;
  border: 2px solid;
  border-top-width: 5px;
  border-left-width: 3px;
  width: 20px;
  padding: 2px 4px 6px 1px;
  direction: rtl;
  margin-left: 40px;
}
.absolute {
  position: absolute;
  grid-column: 1 / 2;
}
.content {
  float: right;
  width: 20px;
  height: 40px;
  background: green;
}
.content:nth-child(2) {
  background: grey;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
