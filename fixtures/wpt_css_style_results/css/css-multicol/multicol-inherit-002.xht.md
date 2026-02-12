# css/css-multicol/multicol-inherit-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-inherit-002.xht"
}
```

## style[0]

```css
<![CDATA[
  body > div
  {
  background-color: blue;
  column-count: 3;
  column-gap: 1em;
  font: 1.25em/1 Ahem;
  width: 32em;

  /*
  N == 3;

    32em
  -  2em : 2 column gaps
 ==========
    30em : available width of the 3 inner multi-column elements

  So, W == 10em;

    30em
  -  6em : 3 mult by 2em: horizontal margin of 3 inner multi-column elements
  -  6em : 3 mult by 2em: horizontal column-gaps of all 3 inner multi-column elements
 ==========
    18em : 6em for each of the 3 column boxes without their own horizontal margins
    So, each column boxes of inner multi-column elements is 2em wide.
  */
  }

  div
  {
  color: black;
  margin: 1em;
  orphans: 1;
  widows: 1;
  }

  div > div
  {
  background-color: yellow;
  column-count: inherit;
  column-gap: 1em;
  }

  div > div:first-child {margin-top: 0;}
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
