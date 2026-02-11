# css/CSS2/tables/table-visual-layout-026a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/table-visual-layout-026a.xht"
}
```

## style[0]

```css
<![CDATA[
  table#red-overlapped
  {
  border-collapse: separate;
  border-spacing: 0px;
  table-layout: fixed;
  }

  col {width: 50px;}

  td
  {
  background-color: red;
  height: 50px;
  padding: 0px 50px 0px 0px;
  }

  div#overlapping-green
  {
  background-color: green;
  bottom: 100px;
  height: 100px;
  left: auto;
  position: relative;
  width: 100px;
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
