# css/css-multicol/multicol-span-all-margin-nested-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-margin-nested-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div#multi-column
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

  h4, div#child
  {
  background-color: black;
  color: black;
  font: inherit;
  margin: 0;
  }

  h4
  {
  margin: 1em 0;
  width: 11em;

  column-span: all;
  }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
