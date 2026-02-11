# css/CSS2/tables/anonymous-table-box-width-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/anonymous-table-box-width-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div#overlapped-red {
    background-color: red;
    height: 100px;
    position: absolute;
    width: 100px;
    z-index: -1;
  }

  table#overlapping-green {
    border-bottom: green solid 100px;
    border-spacing: 0;
  }

  caption {width: 100px;}

  td {padding: 0;}

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
