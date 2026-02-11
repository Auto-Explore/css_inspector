# css/CSS2/tables/padding-applies-to-013a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/padding-applies-to-013a.xht"
}
```

## style[0]

```css
<![CDATA[
  div#red-overlapped
  {
  background-color: red;
  height: 100px;
  position: absolute;
  width: 100px;
  z-index: -1;
  }

  div#green-overlapping-table
  {
  background-color: green;
  display: table;
  padding: 25px;
  table-layout: fixed;
  width: 50px;
  }

  div.table-row {display: table-row;}

  div.table-cell
  {
  display: table-cell;
  height: 25px;
  }
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
