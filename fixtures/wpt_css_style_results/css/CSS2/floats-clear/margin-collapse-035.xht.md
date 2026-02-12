# css/CSS2/floats-clear/margin-collapse-035.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/margin-collapse-035.xht"
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

  div#floated-left
  {
  float: left;
  height: 1px;
  }

  div#floated-right
  {
  float: right;
  height: 1px;
  }

  div#clear-both {clear: both;}

  div#following-sibling {margin-top: 99px;}
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
