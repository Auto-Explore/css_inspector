# css/CSS2/margin-padding-clear/margin-collapse-039.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-039.xht"
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

  div#child-of-cleared-left
  {
  background-color: green;
  color: green;
  height: 50px;
  margin-top: 50px;
  width: 100px;
  }

  /*
  The idea is to have div#cleared-left's margin-top collapse
  with div#child-of-cleared-left's margin-top. The margin collapsing
  should occur before calculating clearance.
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
