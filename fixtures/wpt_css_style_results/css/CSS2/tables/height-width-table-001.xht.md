# css/CSS2/tables/height-width-table-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/height-width-table-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div#overlapped-red-reference
  {
  background-color: red;
  height: 100px;
  position: absolute;
  width: 100px;
  z-index: -1;
  }

  div#overlapping-green-test
  {
  background-color: green;
  border: green solid 25px;
  border-collapse: separate;
  display: table;
  height: 50px;
  width: 50px;
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
