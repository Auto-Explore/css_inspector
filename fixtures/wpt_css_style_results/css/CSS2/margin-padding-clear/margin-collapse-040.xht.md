# css/CSS2/margin-padding-clear/margin-collapse-040.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-040.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 8px;}

  p {margin: 1em 0em;}

  div#overlapped-red
  {
  background-color: red;
  height: 100px;
  position: absolute;
  width: 100px;
  z-index: -1;
  }

  div#floated-left
  {
  background-color: green;
  color: green;
  float: left;
  font: 1em/1.25 serif;
  padding: 15px;
  /*

    15px : padding-top
  +
    20px : line box height
  +
    15px : padding-bottom
   =====
    50px

  */
  width: 70px;
  }

  div#cleared-left {clear: left;}

  table
  {
  border-collapse: separate;
  border-spacing: 0px;
  margin-top: 50px;
  }

  td
  {
  background-color: green;
  color: green;
  height: 50px;
  padding: 0px;
  width: 100px;
  }

  /*
  The idea is to have div#cleared-left's margin-top collapse
  with the table's margin-top. The margin collapsing
  should occur before calculating clearance.
  */
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
