# css/CSS2/positioning/dynamic-top-change-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/dynamic-top-change-005.xht"
}
```

## style[0]

```css
<![CDATA[
    .testDiv { position: relative; width: 100px; height: 100px; }
    #green { top: 100px; background: green; }
    #red { top: inherit; background: red; display: block; }
    #parent { position: relative; }
    body > p { position: absolute; font-size: medium; }
    #grandparent { position: absolute; top: 0; }
    body { font-size: 0; line-height: 0; position: relative; }
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
