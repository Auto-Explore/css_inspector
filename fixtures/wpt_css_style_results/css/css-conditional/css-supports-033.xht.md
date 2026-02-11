# css/css-conditional/css-supports-033.xht

```json
{
  "format_version": 3,
  "file": "css/css-conditional/css-supports-033.xht"
}
```

## style[0]

```css
<![CDATA[
    html { background-color: red }
    @supports not ({ something @with [ balanced ] parens }) {
      html { background-color: green }
    }
  ]]>
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
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
