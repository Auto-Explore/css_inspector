# css/css-multicol/multicol-width-invalid-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-width-invalid-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-color: yellow;
  border: gray solid 1em;
  color: black;
  font: 1.25em/1 Ahem;
  width: 12em;

  column-gap: 0;
  column-width: bzzt; /* this value is invalid */
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
