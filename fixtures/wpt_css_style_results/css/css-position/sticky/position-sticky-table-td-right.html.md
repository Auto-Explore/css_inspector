# css/css-position/sticky/position-sticky-table-td-right.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-table-td-right.html"
}
```

## style[0]

```css

table {
    border-collapse: collapse;
    margin-left: 150px;
}

td {
  padding: 0;
}

td > div {
  height: 50px;
  width: 50px;
}

.group {
  position: relative;
  width: 250px;
  height: 150px;
}

.scroller {
  position: relative;
  width: 200px;
  height: 100px;
  overflow-x: auto;
  overflow-y: hidden;
}

.postpadding {
  height: 10px;
  width: 500px;
}

.indicator {
  position: absolute;
  background-color: red;
  top: 0;
  height: 50px;
  width: 50px;
}

.sticky {
  position: sticky;
  right: 25px;
  background-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
