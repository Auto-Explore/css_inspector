# css/CSS2/normal-flow/inline-block-replaced-width-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/inline-block-replaced-width-008.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  height: 300px;
  width: 600px;
  }

  svg#overlapped-red
  {
  display: inline-block;
  vertical-align: top;
  }

  div#overlapping-green
  {
  background-color: green;
  bottom: 300px;
  height: 300px;
  position: relative;
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
