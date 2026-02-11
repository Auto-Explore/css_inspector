# css/css-fonts/font-size-adjust-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-size-adjust-005.xht"
}
```

## style[0]

```css
<![CDATA[
  div#zero-value-test
  {
  color: red;
  font: 100px/1 Ahem; /* computes to 100px/100px */
  font-size-adjust: 0;
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
