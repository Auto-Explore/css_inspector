# css/CSS2/floats-clear/float-non-replaced-width-013.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/float-non-replaced-width-013.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      background-color: red;
      float: left;  /* or display: inline-block; */  /* or position: absolute; */
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
