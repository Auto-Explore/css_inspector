# css/CSS2/tables/height-width-table-001b.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/height-width-table-001b.xht"
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
  border-collapse: separate;
  display: table;
  height: 50px;
  padding: 25px;
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
