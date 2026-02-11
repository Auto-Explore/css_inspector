# css/CSS2/floats-clear/margin-collapse-clear-016.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/margin-collapse-clear-016.xht"
}
```

## style[0]

```css
<![CDATA[
  body
  {
  background-color: white;
  }

  #parent-block
  {
  background-color: red;
  margin-bottom: 0px;
  }

  #sibling
  {
  background-color: green;
  height: 100px;
  }

  #element-without-clearance-applied
  {
  clear: both;
  margin-top: 100px;
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
