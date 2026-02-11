# css/css-multicol/multicol-rule-color-inherit-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-color-inherit-002.xht"
}
```

## style[0]

```css
<![CDATA[
  div#parent
  {
  color: red;
  font: 20px/1 Ahem;
  }

  div.child
  {
  color: green;
  display: inline-block;
  font-size: 1em;
  width: 5em;

  column-count: 3;
  column-gap: 1em;
  column-rule-color: inherit;
  column-rule-style: solid;
  column-rule-width: 1em;
  }

  /*

  N == 3;

  W == max(0, (available-width - ((N - 1) * column-gap)) / N);
  W == max(0, (5em - ((3 - 1) * 1em)) / 3);
  W == max(0, (5em - (2 * 1em)) / 3);
  W == max(0, (5em - 2em) / 3);
  W == max(0, 3em / 3);
  W == max(0, 1em);
  W == 1em;

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
