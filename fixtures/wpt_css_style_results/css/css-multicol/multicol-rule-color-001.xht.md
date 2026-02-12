# css/css-multicol/multicol-rule-color-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-color-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  border: gray solid 1em;
  color: lime;
  font: 1.25em/1 Ahem;
  orphans: 1;
  widows: 1;
  width: 14em;

  column-count: 4;
  column-gap: 1em;
  column-rule-color: blue;
  column-rule-style: solid;
  column-rule-width: 1em;
  }

  /*

  N == 4;

  W == max(0, (available-width - ((N - 1) * column-gap)) / N);
  W == max(0, (14em - ((4 - 1) * 1em)) / 4);
  W == max(0, (14em - (3 * 1em)) / 4);
  W == max(0, (14em - (3em)) / 4);
  W == max(0, (11em) / 4);
  W == max(0, 2.75em);
  W == 2.75em;

  */

  span {color: black;}
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
