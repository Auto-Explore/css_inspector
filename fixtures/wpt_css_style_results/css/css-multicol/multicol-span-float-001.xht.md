# css/css-multicol/multicol-span-float-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-float-001.xht"
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
  margin-left: 5em;
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

  span
  {
  display: block;
  font: inherit;
  margin: 0;
  width: 11em;

  column-span: all;
  }

  span:first-child
  {
  background-color: pink;
  color: pink;
  float: right;
  }

  span + span {color: black;}
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
      "message": "Invalid value for property “background-color”.",
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
