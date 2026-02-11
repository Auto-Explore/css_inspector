# css/css-multicol/multicol-inherit-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-inherit-001.xht"
}
```

## style[0]

```css
<![CDATA[
  body
  {
  font: 1.25em/1 Ahem;
  width: 34em;
  }

  body > div
  {
  column-count: 3;
  column-gap: 1em;

  /*
  N == 3;

    34em : available width
  -  2em : horizontal margin of external div
  -  2em : 2 column gaps
 ==========
    30em

  So, W == 10em;

    30em
  -  6em : 3 mult by 2em: horizontal margin of 3 internal divs
 ==========
    24em : 8em for each of the 3 column boxes without their horizontal margins

  */
  }

  div
  {
  background-color: yellow;
  color: black;
  margin: 0 1em 1em;
  orphans: 1;
  widows: 1;
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
