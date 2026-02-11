# css/css-multicol/multicol-collapsing-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-collapsing-001.xht"
}
```

## style[0]

```css
<![CDATA[
  body > div
  {
  background-color: black;
  border: black solid 1px;
  font: 1.25em/1 Ahem;
  width: 10em;
  }

  div > div
  {
  background-color: yellow;
  color: black;
  margin: 1em;
  width: 8em;
  orphans: 1;
  widows: 1;

  column-count: 3;
  column-gap: 1em;
  }

  h4
  {
  font: inherit;
  margin: 1em 0 0;
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
