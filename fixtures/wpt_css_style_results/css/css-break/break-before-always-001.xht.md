# css/css-break/break-before-always-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-break/break-before-always-001.xht"
}
```

## style[0]

```css
<![CDATA[
    .multicol {
      columns: 2;
      color: blue;
      font-weight: bold;
    }
    .break {
      break-before: always;
    }
    .control {
      break-before: column;
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
