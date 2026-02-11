# css/CSS2/linebox/border-padding-bleed-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/border-padding-bleed-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  color: red;
  font: 40px/1 Ahem;
  }

  span
  {
  background-color: green; /* so that padding-top area is painted green */
  border-top: green solid 15px;
  color: green;
  padding-top: 25px;
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
