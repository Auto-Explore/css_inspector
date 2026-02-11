# css/CSS2/syntax/escapes-011.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/escapes-011.xht"
}
```

## style[0]

```css

   p { color: green; color: \r\e\d; } /* '\r\e\d' is 'r^N^M', which isn't valid */
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
