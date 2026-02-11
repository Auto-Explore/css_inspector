# css/css-writing-modes/clip-rect-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/clip-rect-vrl-002.xht"
}
```

## style[0]

```css
<![CDATA[
  img#test
    {
      clip: rect(0px, 50px, 50px, 0px);
      position: absolute;
      writing-mode: vertical-rl;
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
