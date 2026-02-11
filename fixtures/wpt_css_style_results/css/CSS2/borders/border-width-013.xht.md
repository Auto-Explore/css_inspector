# css/CSS2/borders/border-width-013.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/borders/border-width-013.xht"
}
```

## style[0]

```css
<![CDATA[
  #parent
  {
  height: 300px;
  width: 400px;
  }

  #child
  {
  border-color: red;
  border-style: solid;
  border-width: 0px;
  border-width: 8%;
  width: 200px;
  }
  ]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-width”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
