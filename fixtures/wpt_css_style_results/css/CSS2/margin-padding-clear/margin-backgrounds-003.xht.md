# css/CSS2/margin-padding-clear/margin-backgrounds-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-backgrounds-003.xht"
}
```

## style[0]

```css
<![CDATA[
  div#parent
  {
  background-color: green;
  border-top: green solid 2px;
  /*
  This border-top's sole purpose is to prevent margin
  collapsing between itself and the p's margin-bottom
  */
  height: 98px;
  width: 100px;
  }

  div#child
  {
  background-color: red;
  margin: 49px 50px;
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
