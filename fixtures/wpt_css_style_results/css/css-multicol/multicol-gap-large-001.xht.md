# css/css-multicol/multicol-gap-large-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-gap-large-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-color: yellow;
  border: gray solid 1em;
  color: black;
  font: 1.25em/1 Ahem;
  orphans: 1;
  widows: 1;
  width: 11em;

  column-count: 4;
  column-gap: 4em;

  /*
  N == 4;
  W == 0em;
  */

  /*
  In this test, the content of first 3 column
  boxes extend into middle of column-gap and
  content of the 4th column box extend outside the
  right edge of multi-column. The gray border-right of
  multi-column elemen overlaps partially the 3rd
  column-gap.
  */
  }

  span {color: blue;}
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
