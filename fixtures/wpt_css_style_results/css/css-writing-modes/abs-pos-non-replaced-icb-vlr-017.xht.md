# css/css-writing-modes/abs-pos-non-replaced-icb-vlr-017.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-icb-vlr-017.xht"
}
```

## style[0]

```css
<![CDATA[
  object#overlapping-green
    {
      height: 116px;
      width: 500px; /* 60% of 500px == 300px (offset of green square) */
      vertical-align: top;
    }

  div#red-overlapped-reference
    {
      background-color: red;
      bottom: 116px;
      height: 100px;
      left: 300px;
      position: relative;
      width: 100px;
      z-index: -1;
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
