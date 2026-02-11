# css/css-multicol/multicol-span-all-margin-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-margin-002.xht"
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

  W == 2;

  */
  }

  h4
  {
  font: inherit;
  margin: 1em 0;
  }

  h4#orange
  {
  background: orange;
  color: orange;
  /*
  In this test, the glyphs "or" are painted into 4th column box
  and the glyphs "ang" are painted in the overflow area.
  */
  }

  h4#black
  {
  background: black;
  color: black;

  column-span: all;
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
