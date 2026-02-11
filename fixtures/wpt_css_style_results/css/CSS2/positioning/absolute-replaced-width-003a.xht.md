# css/CSS2/positioning/absolute-replaced-width-003a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-width-003a.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  height: 225px;
  width: 450px;
  }

  svg#overlapped-red {position: absolute;}

  div#overlapping-green
  {
  background-color: green;
  height: 150px;
  left: auto;
  position: absolute;
  top: auto;
  width: 300px;
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
