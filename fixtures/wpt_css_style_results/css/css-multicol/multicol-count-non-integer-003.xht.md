# css/css-multicol/multicol-count-non-integer-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-count-non-integer-003.xht"
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
  width: 12em;

  column-count: 4;
  column-count: 2.0; /* invalid; must be an integer */
  column-gap: 0;
  }

  span {color: blue;}
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
