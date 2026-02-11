# css/CSS2/floats-clear/margin-collapse-clear-012.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/margin-collapse-clear-012.xht"
}
```

## style[0]

```css
<![CDATA[
  #rel-pos-wrapper {position: relative;}

  #parent-lime
  {
  background-color: lime;
  border-top: black solid 1px;
  width: 50%;
  }

  #float-left-blue
  {
  background-color: blue;
  float: left;
  height: 100px;
  width: 100px;
  }

  #clear-left
  {
  clear: left;
  margin-bottom: 80px;
  margin-top: 40px;
  }

  #following-sibling {margin-bottom: 140px;}

  #next-yellow
  {
  background-color: yellow;
  height: 100px;
  }

  .ref-overlapped-red
  {
  background-color: red;
  position: absolute;
  z-index: -1;
  }

  #ref1
  {
  height: 200px;
  top: 1px;
  width: 50%;
  }

  #ref2
  {
  height: 100px;
  top: 201px;
  width: 100%;
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
