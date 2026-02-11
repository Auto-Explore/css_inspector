# css/css-conditional/css-supports-012.xht

```json
{
  "format_version": 3,
  "file": "css/css-conditional/css-supports-012.xht"
}
```

## style[0]

```css
<![CDATA[
    @supports (color: red) and (color: green) and (color: blue) {
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
