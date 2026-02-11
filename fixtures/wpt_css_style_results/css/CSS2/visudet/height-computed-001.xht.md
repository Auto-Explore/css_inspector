# css/CSS2/visudet/height-computed-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visudet/height-computed-001.xht"
}
```

## style[0]

```css
<![CDATA[
  body {background: white; color: black; font-size: 14px}
  span {display: inline-block}
  #container {background: green; height: 70px; min-height: 140px}
  #child1 {background: #66F; width: 70px; height: inherit} /* Inherits 70px */
  #child2 {background: #AAF; width: 70px; height: 70px}
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
