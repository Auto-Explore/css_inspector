# css/CSS2/floats-clear/margin-collapse-clear-014.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/margin-collapse-clear-014.xht"
}
```

## style[0]

```css
<![CDATA[
  #rel-pos-wrapper {position: relative;}

  #parent-lime
  {
  background-color: lime;
  width: 50%;
  }

  #preceding-sibling-aqua
  {
  background-color: aqua;
  height: 60px;
  margin-bottom: 40px;
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
  margin-top: 120px;
  }

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
  top: 0px;
  width: 50%;
  }

  #ref2
  {
  height: 100px;
  top: 200px;
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
