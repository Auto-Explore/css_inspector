# css/css-multicol/multicol-span-all-margin-bottom-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-margin-bottom-001.xht"
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
  width: 8em;

  column-count: 4;
  column-gap: 0em;

  /*

  N == 4;

  W == 2em;

  */
  }

  h4
  {
  font: inherit;
  margin: 1em 0;
  }

  h4#black
  {
  background: black;
  color: black;

  column-span: all;
  }

  h4#orange
  {
  background: orange;
  color: orange;
  /*
  In this test, the glyphs "or" are painted into 1st column box.
  Per spec, content in the normal flow that extends into column
  gaps (e.g., long words or images) is not clipped to the column
  box. However, the glyphs "ang" are overwritten by <span>s, so
  they're not visible.
  */
  }

  span {color: blue;}

  span + span {color: pink;}
  ]]>
```

```json
{
  "errors": 5,
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
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
