# css/css-multicol/multicol-height-block-child-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-height-block-child-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-color: blue;
  font: 20px/1 Ahem;
  height: 8em;
  width: 14em;

  column-count: 2;
  column-gap: 2em;

  /*

  N == 2;

  W == 6em;

  H == 8em;

  */
  }

  div#outer
  {
  color: black;
  column-fill: auto;
  }

  div#inner
  {
  color: orange;
  column-fill: balance;
  }
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
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
