# css/css-multicol/multicol-count-computed-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-count-computed-003.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background: yellow;
  border: gray solid 1em;
  color: black;
  font: 1.25em/1 Ahem;
  orphans: 1;
  widows: 1;
  width: 13em;

  column-count: 3;
  column-gap: 5em;
  column-rule-color: blue;
  column-rule-style: solid;
  column-rule-width: 1.5em;
  }

  /*
  (11)  if (column-width = auto) and (column-count != auto) then
  (12)    N := column-count;
  (13)    W := max(0, (available-width - ((N - 1) * column-gap)) / N);
  (14)  exit;

  So, the used column-count in this test is 3.

  W := max(0, (available-width - ((N - 1) * column-gap)) / N);
  W == max(0, (13em - ((3 - 1) * 5em)) / 3);
  W == max(0, (13em - (2 * 5em)) / 3);
  W == max(0, (13em - (10em)) / 3);
  W == max(0, (3em) / 3);
  W == max(0, 1em);
  W == 1em;

  So, the used column-width in this test is 1em.
  */

  #pink {color: pink;}
  #orange {color: orange;}
  #purple {color: purple;}
  #gray {color: gray;}

  /*
  Since
  "
  content that extends outside column boxes visibly overflows and is not clipped to the column box.
  "
  https://drafts.csswg.org/css-multicol-1/#overflow-inside-multicol-elements
  this causes the right-half (0.5em) of the 'K' glyph to
  overlap the right-half (0.75em) of the 1st blue column-rule.
  Same thing should happen to the 'N' glyph of 'ORAN'.

  Because no inline content should be rendered into the
  3rd column box, this causes the 2nd column rule not
  been rendered because
  "
  Column rules are only drawn between two columns that
  both have content.
  "
  http://www.w3.org/TR/css3-multicol/#column-gaps-and-rules
  */
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
