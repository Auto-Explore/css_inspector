# css/css-multicol/multicol-span-none-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-none-001.xht"
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

  column-count: 2;
  column-gap: 1em;

  /*

  N == 2;

  W == 5em;

  */
  }

  h4
  {
  background: black;
  color: black;
  font: inherit;
  margin: 0;
  width: 11em;

  column-span: none;
  }

  #column1-top {color: blue;}

  #column1-bottom {color: orange;}

  #column2-top {color: pink;}

  #column2-bottom {color: yellow;}
  ]]>
```

```json
{
  "errors": 4,
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
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
