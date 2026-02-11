# css/css-writing-modes/vertical-alignment-slr-031-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/vertical-alignment-slr-031-ref.xht"
}
```

## style[0]

```css
<![CDATA[
    img
    {
      vertical-align: top;
    }

    img
    {
      padding-left: 90px; /* = the position of first orange square */
    }

    img + br + img
    {
      padding-left: 60px; /* = the position of second orange square*/
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
