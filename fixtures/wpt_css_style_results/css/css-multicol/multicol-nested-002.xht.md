# css/css-multicol/multicol-nested-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-nested-002.xht"
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

  column-gap: 1em;
  column-width: 8em;
  }

  /*

  N == max(1, floor((available-width + column-gap) / (column-width + column-gap)));
  N == max(1, floor((32em + 1em) / (8em + 1em)));
  N == max(1, floor(33em / 9em));
  N == max(1, floor(3.6));
  N == max(1, 3);
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
  orphans: 1;
  widows: 1;
  }

  div > div {margin: 0 1em 1em;}

  div + div {color: blue;}

  div + div + div {color: pink;}
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
