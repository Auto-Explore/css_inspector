# css/css-multicol/multicol-count-non-integer-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-count-non-integer-002.xht"
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
  column-count: 1.9; /* invalid; must be an integer */
  column-gap: 0;
  }

  span {color: blue;}
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
