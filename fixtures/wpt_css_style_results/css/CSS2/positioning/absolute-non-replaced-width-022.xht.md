# css/CSS2/positioning/absolute-non-replaced-width-022.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-width-022.xht"
}
```

## style[0]

```css
<![CDATA[
  body
  {
  direction: rtl;
  margin: 8px;
  }

  p {direction: ltr;}

  div#outer-abs-pos
  {
  background-color: red;
  font: 30px/4 Ahem;
  left: auto;
  position: absolute;
  right: auto;
  /*
  right is set to static position: it should be 8px from
  the right-hand side of document box (at body's margin-right)
  */
  width: auto;
  }

  div#inner-inline-block
  {
  background-color: green;
  display: inline-block;
  max-width: 4em;
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
