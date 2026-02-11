# css/css-writing-modes/vertical-alignment-vrl-022-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/vertical-alignment-vrl-022-ref.xht"
}
```

## style[0]

```css
<![CDATA[
    img
    {
      padding-left: 100px;
      vertical-align: top;
    }
    img + br + img
    {
      padding-left: 180px; /* 100 px (padding-left) + 80 px (the position of small blue box)*/
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
