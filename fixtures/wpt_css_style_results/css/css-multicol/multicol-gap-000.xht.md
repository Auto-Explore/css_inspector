# css/css-multicol/multicol-gap-000.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-gap-000.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  font: 1.25em/1 Ahem;
  margin: 1em 0;
  width: 30em;
  }

  div#test
  {
  background-color: yellow;
  orphans: 1;
  widows: 1;

  column-width: 10em;
  column-gap: 10em;
  }

  span {color: yellow;}
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
