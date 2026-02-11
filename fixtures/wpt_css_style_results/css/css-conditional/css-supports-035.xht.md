# css/css-conditional/css-supports-035.xht

```json
{
  "format_version": 3,
  "file": "css/css-conditional/css-supports-035.xht"
}
```

## style[0]

```css
<![CDATA[
    html { background-color: green }
    @supports not ({ something @with (unbalanced parens }) {
      html { background-color: red }
    }
    /* parser still looking for second close parenthesis */
    html { background-color: red }
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
