# css/CSS2/normal-flow/inline-block-non-replaced-width-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/inline-block-non-replaced-width-005.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      background-color: red;
      display: inline-block;  /* or float: left; */  /* or position: absolute; */
      height: 100px;
      overflow: scroll;
    }

  img
    {
      height: 100%;
      vertical-align: bottom;
      /*
      This 'vertical-align: bottom' declaration is not part of the test.
      We 'baseline-align' the image at the bottom of the line box so
      that the vertical scrollbar remains inactive.
      */
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
