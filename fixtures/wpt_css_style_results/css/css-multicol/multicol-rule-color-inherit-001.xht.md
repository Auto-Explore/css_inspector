# css/css-multicol/multicol-rule-color-inherit-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-color-inherit-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div#parent
  {
  column-rule-color: green;
  column-rule-style: none;
  font: 1.25em/1 Ahem;
  width: 17em;
  }

  /*

  N == 3;

  W == max(0, (available-width - ((N - 1) * column-gap)) / N);
  W == max(0, (17em - ((3 - 1) * 1em)) / 3);
  W == max(0, (17em - (2 * 1em)) / 3);
  W == max(0, (17em - 2em) / 3);
  W == max(0, 15em / 3);
  W == max(0, 5em);
  W == 5em;

  The height of column rule depends on number of line boxes in
  each outer column box which depends on number of line boxes
  in each inner column box.

  N == 3;

  W == max(0, (available-width - ((N - 1) * column-gap)) / N);
  W == max(0, (5em - ((3 - 1) * 1em)) / 3);
  W == max(0, (5em - (2 * 1em)) / 3);
  W == max(0, (5em - 2em) / 3);
  W == max(0, 3em / 3);
  W == max(0, 1em);
  W == 1em;

  */

  div
  {
  color: red; /* both div#parent and div.child have and use 'color: red' */
  orphans: 1;
  widows: 1;

  column-count: 3;
  column-gap: 1em;
  column-rule-width: 1em;
  }

  div.child
  {
  column-rule-color: inherit;
  column-rule-style: solid;
  font-size: 1em;
  }
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
