# css/css-multicol/multicol-fill-auto-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-fill-auto-003.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-image: url("support/red20x20.png");
  background-repeat: no-repeat;
  background-position: 2em 4em;
  border: green solid 1em;
  color: green;
  font: 1.25em/1 Ahem;
  height: 6em;
  width: 21em;

  column-count: 2;
  column-fill: auto;
  column-gap: 1em;
  column-rule: yellow solid 1em;
  }
  ]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
