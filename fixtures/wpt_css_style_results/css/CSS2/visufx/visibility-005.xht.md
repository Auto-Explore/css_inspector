# css/CSS2/visufx/visibility-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visufx/visibility-005.xht"
}
```

## style[0]

```css
<![CDATA[
  div {visibility: hidden;}

  span
  {
  color: red;
  font: 100px/1 Ahem;
  }

  span#testpassed
  {
  color: green;
  visibility: visible;
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
