# css/CSS2/margin-padding-clear/margin-collapse-032.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-032.xht"
}
```

## style[0]

```css
<![CDATA[
  div#grand-parent-block-container
  {
  background-color: red;
  border: black solid 3px;
  /* Such border prevents margin collapsing with the body vertical margins */
  }

  div#parent-block-container
  {
  background-color: green;
  color: white;
  padding: 2%;
  }

  div#child-block {margin: 100px 0px;}

  /*
  In this testcase, div#child-block's vertical (top and bottom)
  margins should collapse with div#parent-block-container's vertical
  (top and bottom) margins because div#child-block has no used
  padding-top and no used padding-bottom. On the other hand,
  div#parent-block-container's vertical (top and bottom) margins
  should NOT collapse with div#grand-parent-block-container's vertical
  (top and bottom) margins because div#parent-block-container, after
  resolving percentage values, has an used padding-top value and an used
  padding-bottom value.
  */
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
