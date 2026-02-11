# css/css-conditional/css-supports-022.xht

```json
{
  "format_version": 3,
  "file": "css/css-conditional/css-supports-022.xht"
}
```

## style[0]

```css
<![CDATA[
    html { background-color: red }
    @supports (unknown:) or (color: green) {
      html { background-color: green }
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
