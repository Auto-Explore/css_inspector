# css/CSS2/margin-padding-clear/margin-backgrounds-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-backgrounds-002.xht"
}
```

## style[0]

```css
<![CDATA[
  p {margin: 1em 0em;}

  div#parent
  {
  background-color: green;
  height: 100px;
  width: 100px;
  }

  div#child
  {
  background-color: red;
  height: 1px;
  /*
  'height: 1px' is set intentionally so that the
  top margin (16px) and bottom margin (99px) of such child
  box do not collapse through the element.
  */
  margin: 16px 100px 99px 0px;
  /*
  'margin-top: 16px' correspond to the margin-bottom of the
  <p>. The margin-top of the child box collapses with the
  margin-top of div#parent and then the collapsed margin-top
  of div#parent then collapses in its turn with the
  margin-bottom of the <p>.
  */
  width: auto;
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
