# css/css-multicol/multicol-block-no-clip-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-block-no-clip-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  border: gray solid 1em;
  font: 1.25em/1 Ahem;
  orphans: 1;
  widows: 1;
  width: 11em;

  column-count: 4;
  column-gap: 1em;
  }

  h4
  {
  background: black;
  color: black;
  font: inherit;
  margin: 0;
  }

  #first-column {color: blue;}

  #second-column {color: orange;}

  #third-column, #fourth-column {color: white;}
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
