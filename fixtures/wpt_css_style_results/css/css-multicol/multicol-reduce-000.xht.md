# css/css-multicol/multicol-reduce-000.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-reduce-000.xht"
}
```

## style[0]

```css
<![CDATA[
  div#test, div#reference
  {
  background-color: yellow;
  border: black solid thin;
  color: orange;
  font: 1.25em/1 Ahem;
  margin-bottom: 0.5em;
  width: 30em;
  }

  div#test
  {
  column-gap: 0;
  column-width: 201px;

  /*

  N == max(1, floor((U + column-gap)/(column-width + column-gap)))
  N == max(1, floor((600px + 0px)/(201px + 0)))
  N == max(1, floor(600px/201px))
  N == max(1, floor(2.985))
  N == max(1, 2)
  N == 2;

  W == max(0, ((U + column-gap)/N - column-gap))
  W == max(0, ((600px + 0px)/2 - 0px))
  W == max(0, ((600px)/2 - 0px))
  W == max(0, (300px - 0px))
  W == max(0, 300px)
  W == 300px;

  */
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
