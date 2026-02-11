# css/css-conditional/css-supports-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-conditional/css-supports-005.xht"
}
```

## style[0]

```css
<![CDATA[
    html { background-color: green }
    @supports (color: rainbow) {
      html { background-color: red }
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
