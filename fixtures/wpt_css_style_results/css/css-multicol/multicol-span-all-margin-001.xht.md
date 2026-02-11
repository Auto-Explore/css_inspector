# css/css-multicol/multicol-span-all-margin-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-margin-001.xht"
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
  overflow: hidden;
  widows: 1;
  width: 8em;

  column-count: 4;
  column-gap: 0em;

  /*

  N == 4;

  W == 2;

  */
  }

  h4
  {
  background: black;
  color: black;
  font: inherit;
  margin: 1em 0;
  width: 11em;

  column-span: all;

  /*
  This is the target of the test: the spanning element's
  width (11em) exceeds the available width of the
  multi-column element. The 3em exceeding such
  width is first clipped at column box boundary.
  */
  }

  span {color: blue;}

  span + span {color: pink;}
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
