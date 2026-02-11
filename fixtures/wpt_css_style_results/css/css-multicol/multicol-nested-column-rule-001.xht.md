# css/css-multicol/multicol-nested-column-rule-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-nested-column-rule-001.xht"
}
```

## style[0]

```css
<![CDATA[
  body > div
  {
  column-rule: blue solid 1em;
  font: 1.25em/1 Ahem;
  width: 26em;
  }

  /*

  N == 3;

  W == max(0, (available-width - ((N - 1) * column-gap)) / N);
  W == max(0, (26em - ((3 - 1) * 1em)) / 3);
  W == max(0, (26em - (2 * 1em)) / 3);
  W == max(0, (26em - 2em) / 3);
  W == max(0, 24em / 3);
  W == max(0, 8em);
  W == 8em;

  So, the first column-rule should be at:

    1.0em : margin-left of outer div
    8.0em : width of 1st column box
    0.0em : (1.0em / 2) - (1.0em / 2) : left edge of 1st column-rule
  =========
    9.0em

  The 2nd column-rule should be at:

    1.0em : margin-left of outer div
    8.0em : width of 1st column box
    1.0em : first column-gap
    8.0em : width of 2nd column box
    0.0em : (1.0em / 2) - (1.0em / 2) : left edge of 2nd column-rule
  =========
   18.0em

  The height of column rule depends on number of line boxes in
  each outer column box which depends on number of line boxes
  in each inner column box. So:

     8em : width of each outer column box
   -
     2em : horizontal margin of each div inside
   =======
     6em : width of each inner multi-column elements

  N == 3;

  W == max(0, (available-width - ((N - 1) * -column-gap)) / N);
  W == max(0, (6em - ((3 - 1) * 1em)) / 3);
  W == max(0, (6em - (2 * 1em)) / 3);
  W == max(0, (6em - 2em) / 3);
  W == max(0, 4em / 3);
  W == max(0, 1.33333em);
  W == 1.33333em;

  So, each duo of 'a', 'm' and 'x' should fill one and only 1 line
  box. There are 8 duos; therefore, the first 2 inner
  column boxes should use 3 line boxes and the 3rd inner
  column box should be using 2 line boxes.

  So, the height of the 2 blue column rules should be 60px.

  */

  div
  {
  background-color: white;
  color: white;
  font-size: 1em;
  margin: 0em 1em;
  orphans: 1;
  widows: 1;

  column-count: 3;
  column-gap: 1em;
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
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
