# css/css-multicol/multicol-rule-double-000.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-double-000.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-color: black;
  color: black;
  font: 3.125em/1 Ahem;
  margin-bottom: 0.2em;
  width: 8.2em;
  }

  div#test
  {
  columns: 2;
  column-gap: 0.2em;
  column-rule: lime double 0.2em;

  /*

  N == 2;

  W == 200px;

  */

  orphans: 1;
  widows: 1;
  }

  span {border-left: lime double 0.2em;}
  ]]>
```

```json
{
  "errors": 3,
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
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
