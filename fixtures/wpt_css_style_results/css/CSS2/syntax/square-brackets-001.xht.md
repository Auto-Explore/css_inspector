# css/CSS2/syntax/square-brackets-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/square-brackets-001.xht"
}
```

## style[0]

```css
<![CDATA[
      p { color: red; }
      [
      p { color: red !important; }
      p { color: red !important; }
      ] p { color: red !important; }
      p { color: green; }
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
      "message": "Invalid selector.",
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
