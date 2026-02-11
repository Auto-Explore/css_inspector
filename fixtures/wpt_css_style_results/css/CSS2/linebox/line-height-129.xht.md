# css/CSS2/linebox/line-height-129.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/line-height-129.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-color: red;
  line-height: 0px;
  }

  img
  {
  margin-bottom: -100px;
  margin-top: 0px;
  vertical-align: bottom; /* or top */
  /*
  By default, images "sit" on the baseline (vertical-align's default value is 'baseline', not 'bottom' and not 'top') and not at the bottom of line boxes. Therefore, if we want to "nullify" the height of line box, then we must set 'vertical-align' to 'bottom' or to 'top'. "In case they [inline boxes] are [vertically-] aligned [to] 'top' or 'bottom', they must be aligned so as to minimize the line box height.": the line box height would not be minimizable to zero if the inline replaced box had been "sitting" on the baseline.
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
