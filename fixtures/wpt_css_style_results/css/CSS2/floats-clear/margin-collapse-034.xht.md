# css/CSS2/floats-clear/margin-collapse-034.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/margin-collapse-034.xht"
}
```

## style[0]

```css
<![CDATA[
  div#overlapped-red
  {
  background-color: red;
  height: 100px;
  position: absolute;
  width: 100px;
  z-index: -1;
  }

  div#overlapping-green-container
  {
  background-color: green;
  width: 100px;
  }

  div#floated-right
  {
  float: right;
  height: 1px;
  }

  div#clear-right {clear: right;}

  div#following-sibling {margin-top: 99px;}
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
