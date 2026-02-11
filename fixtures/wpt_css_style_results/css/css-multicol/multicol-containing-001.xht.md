# css/css-multicol/multicol-containing-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-containing-001.xht"
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
  width: 12em;

  column-count: 3;
  column-gap: 0;
  }

  span
  {
  color: red;
  position: absolute;
  top: -1em;
  }

  /*
  In this test, the initial containing block establishes containing block
  for such red span. Therefore, the 2 short red stripes should be
  positioned out of view, just outside the top edge of viewport.
  */

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
