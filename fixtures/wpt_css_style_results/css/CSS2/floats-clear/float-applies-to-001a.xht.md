# css/CSS2/floats-clear/float-applies-to-001a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/float-applies-to-001a.xht"
}
```

## style[0]

```css
<![CDATA[
  div#table
  {
  display: table;
  table-layout: fixed;
  width: 100%;
  }

  div#table-row-group
  {
  background-color: blue;
  display: table-row-group;
  float: right;
  height: 1in;
  width: 1in;
  }

  div.row {display: table-row;}

  div.cell
  {
  color: blue;
  display: table-cell;
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
