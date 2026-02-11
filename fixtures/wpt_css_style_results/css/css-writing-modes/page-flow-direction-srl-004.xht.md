# css/css-writing-modes/page-flow-direction-srl-004.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/page-flow-direction-srl-004.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: sideways-rl;
    }

  body, div
    {
      width: 100%;
  /* width: 100% will force a page-break in sideways-rl writing-mode */
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
