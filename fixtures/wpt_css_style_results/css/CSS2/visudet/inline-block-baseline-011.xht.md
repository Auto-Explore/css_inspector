# css/CSS2/visudet/inline-block-baseline-011.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visudet/inline-block-baseline-011.xht"
}
```

## style[0]

```css
<![CDATA[
  body {background: white; color: black; font-size: 15px}
  p {white-space: nowrap; line-height: 5}
  span {display: inline-block; overflow: visible;
    /* This inline-block has no line boxes, it's aligned at the bottom margin */
    vertical-align: -0.5em; margin-bottom: 0.5em;
    height: 1em; width: 1em; background: blue}
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
