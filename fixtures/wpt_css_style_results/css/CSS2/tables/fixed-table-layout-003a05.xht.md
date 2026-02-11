# css/CSS2/tables/fixed-table-layout-003a05.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/fixed-table-layout-003a05.xht"
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
  width: 400px;
  }

  td
  {
  border: none;
  padding: 0px 0px 0px 120px;
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
  margin-left: 100px;
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
