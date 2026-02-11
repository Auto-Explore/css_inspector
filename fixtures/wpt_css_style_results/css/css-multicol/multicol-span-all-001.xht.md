# css/css-multicol/multicol-span-all-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-color: yellow;
  border: gray solid 1em;
  color: navy;
  font: 1.25em/1 Ahem;
  orphans: 1;
  widows: 1;
  width: 11em;

  column-count: 4;
  column-gap: 1em;

  /*

  N == 4;

  W == 2em;

  */

  }

  span {color: blue;}

  span + span {color: pink;}

  h4
  {
  background-color: black;
  color: black;
  font: inherit;
  margin: 0;

  column-span: all;
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
