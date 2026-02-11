# css/css-writing-modes/clip-rect-vlr-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/clip-rect-vlr-003.xht"
}
```

## style[0]

```css
<![CDATA[
  img#test
    {
      clip: rect(0px, 50px, 50px, 0px);
      position: absolute;
      writing-mode: vertical-lr;
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
