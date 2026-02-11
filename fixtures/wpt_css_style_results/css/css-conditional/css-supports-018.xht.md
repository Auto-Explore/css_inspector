# css/css-conditional/css-supports-018.xht

```json
{
  "format_version": 3,
  "file": "css/css-conditional/css-supports-018.xht"
}
```

## style[0]

```css
<![CDATA[
    @supports not (not (color: green)) {
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
