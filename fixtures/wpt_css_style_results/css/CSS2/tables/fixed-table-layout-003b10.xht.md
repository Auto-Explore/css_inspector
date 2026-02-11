# css/CSS2/tables/fixed-table-layout-003b10.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/fixed-table-layout-003b10.xht"
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

  td {padding: 0px 24px;}

  td#tested-cell
  {
  background-color: blue;
  border-left: orange solid 0px;
  border-right: orange solid 72px;
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
  "errors": 4,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
