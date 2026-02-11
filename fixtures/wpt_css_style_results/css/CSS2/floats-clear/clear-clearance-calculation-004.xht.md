# css/CSS2/floats-clear/clear-clearance-calculation-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/clear-clearance-calculation-004.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 8px;}

  #overlapped-red
  {
  background-color: red;
  height: 100px;
  left: 8px;
  position: absolute;
  right: 8px;
  top: 108px;
  z-index: -1;
  }

  #firstParg
  {
  height: 25px;
  margin-bottom: 25px;
  margin-top: 0px;
  }

  #floatingParg
  {
  float: left;
  height: 50px;
  margin: 0;
  }

  #lastParg
  {
  background-color: green;
  clear: left;
  height: 100px;
  margin-top: 75px;
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
