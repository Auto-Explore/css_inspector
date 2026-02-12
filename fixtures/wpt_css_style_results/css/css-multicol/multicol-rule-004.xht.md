# css/css-multicol/multicol-rule-004.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-004.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 8px;}

  p
  {
  line-height: 1.25em;
  margin: 1em 0em;
  }

  strong {line-height: 1;}

  div#red-overlapped-reference
  {
  background-color: red;
  height: 100px;
  width: 100px;
  }

  div#test-overlapping-green
  {
  bottom: 100px;
  color: transparent;
  font: 1.25em/1 Ahem; /* equivalent to 20px/1 Ahem */
  height: 5em;
  position: relative;
  right: 80px;
  width: 13em;

  column-count: 2;
  column-fill: balance;
  column-gap: 5em;
  column-rule: green solid 5em;
  }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
