# css/css-multicol/multicol-clip-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-clip-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  border: gray solid 1em;
  font: 1.25em/1 serif;
  width: 11em;
  }

  img
  {
  position: relative;
  vertical-align: top;
  }

  img + img {left: 3em;}

  img + img + img {left: 6em;}
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
