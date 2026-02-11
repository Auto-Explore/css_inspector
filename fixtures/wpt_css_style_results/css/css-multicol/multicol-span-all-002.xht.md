# css/css-multicol/multicol-span-all-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-002.xht"
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
  margin: 1em 0 1em 8em;

  column-span: all;
  /*
  Since there is not sufficient space for the
  spanning element, UA may treat the element as
  'column-span: none'; in which case,
  'overflow: hidden' will take care of
  overflowed content out of multi-column box.
  */
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
