# css/CSS2/tables/fixed-table-layout-003a04.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/fixed-table-layout-003a04.xht"
}
```

## style[0]

```css
<![CDATA[
  div, table {font: 1.25em/1.2 serif;}

  table
  {
  border-spacing: 0px;
  border-collapse: separate;
  table-layout: fixed;
  width: 300px;
  }

  td
  {
  border: none;
  padding: 0px 120px 0px 0px;
  }

  td#tested-cell
  {
  background-color: blue;
  color: blue;
  width: 80px;
  }

  div#reference
  {
  background-color: black;
  color: black;
  margin-left: 50px;
  width: 200px;
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
