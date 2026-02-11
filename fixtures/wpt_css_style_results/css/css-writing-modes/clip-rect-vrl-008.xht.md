# css/css-writing-modes/clip-rect-vrl-008.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/clip-rect-vrl-008.xht"
}
```

## style[0]

```css
<![CDATA[
  img#test
    {
      clip: rect(50px, 100px, 100px, 50px);
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
