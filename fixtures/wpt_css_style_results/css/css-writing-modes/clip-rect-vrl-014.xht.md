# css/css-writing-modes/clip-rect-vrl-014.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/clip-rect-vrl-014.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-rl;
    }

  img#test
    {
      clip: rect(50px, 50px, 100px, 0px);
      position: absolute;
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
