# css/CSS2/css1/c71-fwd-parsing-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/css1/c71-fwd-parsing-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  p, div { color: navy; }

  div
  {
  border: navy solid medium;
  margin: 1em 0;
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
