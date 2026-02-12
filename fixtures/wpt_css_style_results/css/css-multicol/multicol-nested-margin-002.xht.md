# css/css-multicol/multicol-nested-margin-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-nested-margin-002.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 0;}

  body > div
  {
  background-color: yellow;
  font: 1.25em/1 Ahem;
  width: 41em;
  /*
  This test requires a viewport width of 820px
  */
  }

  div
  {
  color: yellow;
  font: inherit;
  orphans: 1;
  widows: 1;

  column-count: 3;
  column-gap: 1em;
  }

  div div {margin: 0em 1em;}

  div div:last-child
  {
  color: blue;
  margin: 1em;
  }

  /*

  N == 3;

  W == max(0, (available-width - ((N - 1) * column-gap)) / N);
  W == max(0, (41em - ((3 - 1) * 1em)) / 3);
  W == max(0, (41em - (2 * 1em)) / 3);
  W == max(0, (41em - 2em) / 3);
  W == max(0, 39em / 3);
  W == max(0, 13em);
  W == 13em;

  The height of column rule depends on number of line boxes in
  each outer column box which depends on number of line boxes
  in each inner column box. So:

    13em : width of each outer column box
   -
     2em : horizontal margin of each div inside
   =======
    11em : width of each inner multi-column elements

  N == 3;

  W == max(0, (available-width - ((N - 1) * column-gap)) / N);
  W == max(0, (11em - ((3 - 1) * 1em)) / 3);
  W == max(0, (11em - (2 * 1em)) / 3);
  W == max(0, (11em - 2em) / 3);
  W == max(0, 9em / 3);
  W == max(0, 3em);
  W == 3em;

  So, each duo of 'a', 'm' and 'x' should fill one and only 1
  line box. There are 15 'a' duos and 15 'm' duos; therefore,
  the 3 inner column boxes of each first 2 inner
  multi-column elements should use 5 line boxes.

  The 1st column box of last inner multi-column
  (the blue one with 'x' duos) should have 'x1',
  'x2' and 'x3' duos filling 3 line boxes.

  The 2nd column box of last inner multi-column should
  have 'x4', 'x5' and 'x6' duos filling 3 line boxes.

  The 3rd column box of the last inner multi-column should
  have 'x7' and 'x8' duos filling 2 line boxes.
  */

  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
