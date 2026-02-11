# css/CSS2/margin-padding-clear/margin-collapse-038.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-038.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 8px;}

  p {margin: 8px 0px 0px;}

  div#reference-overlapped-red
  {
  background-color: red;
  height: 200px;
  left: 8px;
  position: absolute;
  right: 8px;
  z-index: -1;
  }

  div#green-grand-parent
  {
  background-color: green;
  border-bottom: green solid 1px;
  }

  div#parent-with-max-height
  {
  margin-bottom: 0px;
  max-height: 99px;
  }

  div#last-child
  {
  height: 300px;
  /*
  The height of parent-with-max-height is constrained by
  its max-height which is smaller than this last-child height
  */
  margin-bottom: 100px;
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
