# css/css-multicol/multicol-rule-large-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-large-001.xht"
}
```

## style[0]

```css
<![CDATA[
  body
  {
  font: 1.25em/1 Ahem;
  margin-left: 1em;
  }

  div
  {
  background-color: red;
  border: gray solid 1em;
  color: lime;
  font-size: 1em;
  orphans: 1;
  widows: 1;
  width: 15em;

  column-count: 4;
  column-gap: 1em;
  column-rule-color: blue;
  column-rule-style: solid;
  column-rule-width: 17em;
  }

  /*

  N == 4;

  W == max(0, (available-width - ((N - 1) * column-gap)) / N);
  W == max(0, (15em - ((4 - 1) * 1em)) / 4);
  W == max(0, (15em - (3 * 1em)) / 4);
  W == max(0, (15em - (3em)) / 4);
  W == max(0, (12em) / 4);
  W == max(0, 3em);
  W == 3em;

  */

  /*
  Since column rules do not take up space, then the test
  should not generate an horizontal scrollbar.
  */
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
