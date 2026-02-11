# css/css-multicol/multicol-break-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-break-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div#test, div#reference
  {
  background-color: yellow;
  margin-bottom: 0.5em;
  font: 1.25em/1 Ahem;
  height: 3em;
  width: 10em;
  }

  div#test
  {
  column-fill: auto;
  column-gap: 0;
  column-width: 2em;

  /*

  N == 5;

  W == 2em;

  */
  }

  div#test > div {break-before: column;}

  img {vertical-align: top;}
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
