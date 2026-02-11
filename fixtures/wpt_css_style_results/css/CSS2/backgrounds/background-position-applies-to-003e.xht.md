# css/CSS2/backgrounds/background-position-applies-to-003e.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-position-applies-to-003e.xht"
}
```

## style[0]

```css
<![CDATA[
  div#table {display: table;}

  div#tfoot
  {
  background-image: url("/css/support/60x60-red.png");
  background-position: bottom right;
  background-repeat: no-repeat;
  display: table-footer-group;
  }

  div.tr {display: table-row;}

  div.td
  {
  display: table-cell;
  height: 60px;
  width: 60px;
  }

  div#overlapping-green
  {
  border-bottom: green solid 60px;
  display: table-cell;
  height: 60px;
  width: 60px;
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
