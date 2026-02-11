# css/css-multicol/multicol-nested-margin-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-nested-margin-001.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 0em;}

  body > div
  {
  font: 1.25em/1 Ahem;
  width: 32em;

  column-count: 3;
  column-gap: 1em;
  }

  /*

  N == 3;

  W == ((available-width + column-gap) / N) - column-gap;
  W == ((32em + 1em) / 3) - 1em;
  W == (33em / 3) - 1em;
  W == (11em) - 1em;
  W == 10em;

  */

  div
  {
  background: yellow;
  color: black;
  margin: 1em;
  orphans: 1;
  widows: 1;
  }

  div > div {margin: 0em 1em;}

  div div:nth-child(2) {color: pink;}

  div div:nth-child(3) {color: blue;}
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
